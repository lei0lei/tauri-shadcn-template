#![allow(warnings)] 
use std::sync::{Arc, RwLock};
use std::thread;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{Sender, Receiver};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;
use tauri::{AppHandle, Manager,Emitter};
use base64;
use std::time::Duration;
use tokio::process::Child; 
use tokio::sync::{oneshot, Mutex, mpsc};
use tauri::{State, WindowEvent};
mod sidecar;
use std::collections::HashMap;
use once_cell::sync::Lazy;
mod plc;
use plc::modbusTCP;
use plc::modbusTCP::{PLC_TX, 
                    ROBOT_TX,
                    start_plc_connect,
                    start_robot_connect,
                    read_register_plc,
                    read_register_robot,
                    write_register_plc,
                    write_register_robot,
                    read_coil,
                    write_coil,
                    start_plc_connection,
                    start_robot_connection,
                    stop_plc_connection,
                    stop_robot_connection,
                    read_multiple_registers_robot
                  };

use sensors::cf3000::{
  rs_CF_RegisterEventCallback,
  rs_CF_StartSample,
  rs_CF_GE_OpenDevice
};
use std::net::SocketAddr;
use std::{path::{Path, PathBuf}};
mod config;
use toml::Value;
mod cameras;
use cameras::hik_camera::{
                        init_mvs_sdk, 
                        enumerate_devices,
                        is_device_accessable,
                        create_handle,
                        destroy_handle,
                        is_device_connected,
                        open_device,
                        close_device,
                        start_grabbing,
                        stop_grabbing,
                        register_callback,
                        get_oneframe_timeout,
                      };
use hikvision::mvs_sdk::types::{MvAccessMode,MvEnumDeviceLayerType,MvFrameOutInfoEx};
use tauri::path::BaseDirectory;
use opencv::{
  core::{Mat, MatTrait, CV_8UC3, CV_8U,Scalar,AlgorithmHint,Vector},
  imgcodecs,
  imgproc,
  prelude::*,
};
use std::fs;
use reqwest::Client;
use tokio::sync::OnceCell;
mod sensors;
use chrono::Local;
// 主流程启动状态
#[derive(Debug, PartialEq,Copy, Clone)]
pub enum SoftwareState{
  START,
  STOP,
}

// 硬件开发模式
#[derive(Debug, PartialEq,Copy, Clone)]
pub enum HardwareDevState{
  ON,
  OFF,
}

lazy_static::lazy_static! {
  pub static ref HARDWARE_DEV_STATE: Arc<Mutex<HardwareDevState>> = Arc::new(Mutex::new(HardwareDevState::OFF));
}

// 硬件连接状态
pub enum HardwareState{
  plc(bool),
  camera(bool),
  sensor(bool),
  robot(bool),
}

lazy_static::lazy_static! {
  pub static ref START_STATE: Arc<std::sync::Mutex<SoftwareState>> = Arc::new(std::sync::Mutex::new(SoftwareState::STOP));
}
lazy_static::lazy_static! {
  pub static ref START_PROCESS_STATE: Arc<Mutex<SoftwareState>> = Arc::new(Mutex::new(SoftwareState::STOP));
}

pub static mut PLC_STATE: HardwareState = HardwareState::plc(false);
pub static mut CAMERA_STATE: HardwareState = HardwareState::camera(false);
pub static mut SENSOR_STATE: HardwareState = HardwareState::sensor(false);
pub static mut ROBOT_STATE: HardwareState =HardwareState::robot(false);

// modbus连接
use lazy_static::lazy_static;
use reqwest::multipart::{Form, Part};

lazy_static! {
  static ref GLOBAL_TX: Arc<Mutex<Option<mpsc::Sender<GeneralRequest>>>> = Arc::new(Mutex::new(None));
}

lazy_static! {
  static ref GLOBAL_SENSOR_TX: Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<SensorsDataRequest>>>> =
      Arc::new(std::sync::Mutex::new(None));
}
// 定义全局变量
pub static GLOBAL_TASK_STATE: Lazy<Arc<tokio::sync::RwLock::<TaskState>>> = Lazy::new(|| {
  Arc::new(tokio::sync::RwLock::new(TaskState::new("EH09".to_string(), "1999_99_99_99".to_string())))
});


// 检测状态
pub struct TaskState {
  pub current_artifact: String, // 当前零件
  pub current_artifact_type: String, //当前型号
  pub current_face: u16, // 当前正在检测的面编号
  pub current_hole: u16, // 当前正在检测的孔编号
  pub holes: HashMap<(u16, u16), HoleState>, // (face_id, hole_id) -> HoleState
}

// 每个孔的状态
#[derive(Clone)]
pub struct HoleState {
  pub face_id: u16,   //面编号
  pub hole_id: u16,   // 孔编号
  pub action1: Vec<f64>, // 动作1的数据
  pub action2: Vec<f64>, // 动作2的数据
  pub action3: Option<Yolov8Result>, // 动作3的检测结果
  pub action4: Option<HoleDiameter>, // 动作4的检测结果，如圆心、直径等
}

#[derive(Clone)]
pub struct Yolov8Result {
  pub detections: Vec<Detection>,
}

#[derive(Clone)]
pub struct HoleDiameter {
  pub nei_center: (f64,f64),
  pub nei_diameter: f64,
  pub wai_center: (f64,f64),
  pub wai_diameter: f64,
}

// 检测结果
#[derive(Clone)]
pub struct Detection {
  pub class_id: u32,
  pub confidence: f64,
  pub bbox: (f64, f64, f64, f64), // x, y, x, y
}

impl TaskState {
  pub fn new(artifact: String, artifact_type: String) -> Self {
    Self { 
      current_artifact: artifact, 
      current_artifact_type: artifact_type,
      current_face: 1,  // 默认值
      current_hole: 1,  // 默认值
      holes: HashMap::new(), // 空的 HashMap
       }
  }

  // 添加孔位
  pub async fn add_hole(&mut self, face_id: u16, hole_id: u16) {
    self.holes.insert((face_id, hole_id), HoleState {
        face_id,
        hole_id,
        action1: Vec::new(),
        action2: Vec::new(),
        action3: None,
        action4: None,
    });
  }

  pub async fn update_action1(&mut self, face_id: u16, hole_id: u16, data: f64) {
    if let Some(hole) = self.holes.get_mut(&(face_id, hole_id)) {
        hole.action1.push(data);
    }
  }

  pub async fn update_action2(&mut self, face_id: u16, hole_id: u16, data: f64) {
      if let Some(hole) = self.holes.get_mut(&(face_id, hole_id)) {
        hole.action2.push(data);
      }
  }

  pub async fn update_action3(&mut self, face_id: u16, hole_id: u16, detection: Yolov8Result) {
      if let Some(hole) = self.holes.get_mut(&(face_id, hole_id)) {
          hole.action3 = Some(detection);
      }
  }

  pub async fn update_action4(&mut self, face_id: u16, hole_id: u16, diameter: HoleDiameter) {
      if let Some(hole) = self.holes.get_mut(&(face_id, hole_id)) {
          hole.action4 = Some(diameter);
      }
  }
  pub async fn get_hole_state(&self, face_id: u16, hole_id: u16) -> Option<HoleState> {
    self.holes.get(&(face_id, hole_id)).cloned()
  }

  pub async fn clear(&mut self) {
    self.holes.clear();
    self.current_face = 0;
    self.current_hole = 0;
  }
}
// generate_detection_result
pub fn generate_detection_result(result: &Yolov8Result) -> bool {
  result.detections.iter().any(|d| d.class_id == 0)
}

// generate_depth_result
pub fn generate_depth_result(action1: &[f64], action2: &[f64], min_val: f64, max_val: f64) -> bool {
  if action1.is_empty() || action2.is_empty() {
    return false; // 避免空数组计算平均值导致错误
  }

  let avg1 = action1.iter().sum::<f64>() / action1.len() as f64;
  let avg2 = action2.iter().sum::<f64>() / action2.len() as f64;
  let diff = (avg1 - avg2).abs();
  // 判断 diff 是否在 [min_val, max_val] 区间内
  (min_val..=max_val).contains(&diff)
}

// generate_diameter_result
// 判断直径是否在指定范围内
pub fn generate_diameter_result(diameter: &HoleDiameter, min_val: f64, max_val: f64) -> bool {
  (min_val..=max_val).contains(&diameter.nei_diameter) && (min_val..=max_val).contains(&diameter.wai_diameter)
}

// 
// 保存路径相关
// 
static BASE_PATH: &str = "D:"; // 你的全局 base_path
fn generate_file_path(paths: &[&str], filename: &str) -> PathBuf {
  let mut dir_path = PathBuf::from(BASE_PATH);
  
  // 依次添加路径参数
  for p in paths {
      dir_path.push(p);
  }

  // 确保目录存在
  if let Err(e) = fs::create_dir_all(&dir_path) {
      eprintln!("Failed to create directory: {:?}", e);
  }

  // 返回完整的文件路径
  dir_path.join(filename)
}

// 数据消息队列(相机、传感器)
pub enum SensorsDataRequest {
  ImageProcess(cameras::hik_camera::FrameInfoSafe,Vec<u8>),
  Cf3000(f64),
}

// 初始化传感器消息队列,回调需要在同步环境中运行
pub fn start_sensor_task(mut rx: std::sync::mpsc::Receiver<SensorsDataRequest>) -> Result<(), String> {
  let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?; // 创建一个 Tokio 运行时

  while let Ok(request) = rx.recv() {
    match request {
      SensorsDataRequest::ImageProcess(frame_info,image_data)=>{
        let log = "[camera] [log] [info]";
        // 获取当前的状态
        let mut state = START_STATE.lock().unwrap();
        if *state != SoftwareState::START {
        } else{
        // 发送图片到前端
          rt.spawn(async move {
              let (resp_tx, resp_rx) = oneshot::channel();
              let tx = GLOBAL_TX.lock().await.clone().unwrap_or_else(|| {
                  panic!("GLOBAL_TX is not initialized. Ensure that start_plc_connect() has been called.");
              });
              // 获取当前位置
              let pos = match read_multiple_registers_robot(256, 3).await {
                Ok(values) if values.len() == 3 => values, 
                _ => {
                    println!("读取寄存器失败，数据不足");
                    return Err("读取寄存器失败".to_string());
                }
              };

            // 如果需要禁用算法，修改此处代码
              tx.send(GeneralRequest::SendImageToFastapi(pos.clone(),frame_info,image_data, resp_tx))
                  .await
                  .map_err(|_| "发送请求失败".to_string());
              // 如果需要禁用算法，修改此处代码
              // tx.send(GeneralRequest::SendImageToFrontend(frame_info,image_data, resp_tx))
              //     .await
              //     .map_err(|_| "发送请求失败".to_string());
              match resp_rx.await {
                Ok(_) => {
                  Ok(())
                }
                Err(e) => {
                  return Err("图片发送请求失败".to_string());
                }
              }
          });
        }
      }

      SensorsDataRequest::Cf3000(data)=>{
        println!("获取传感器数据:{}",data);
        rt.spawn(async move {
          let (resp_tx, resp_rx) = oneshot::channel();
          let tx = GLOBAL_TX.lock().await.clone().unwrap_or_else(|| {
              panic!("GLOBAL_TX is not initialized. Ensure that start_plc_connect() has been called.");
          });

          let pos = match read_multiple_registers_robot(256, 3).await {
            Ok(values) if values.len() == 3 => values, 
            _ => {
                println!("读取寄存器失败，数据不足");
                return Err("读取寄存器失败".to_string());
            }
          };

          tx.send(GeneralRequest::SendSensorDataToFrontend(pos.clone(),data, resp_tx))
              .await
              .map_err(|_| "发送请求失败".to_string());
          match resp_rx.await {
                    Ok(_) => {
                      Ok(())
                    }
                    Err(e) => {
                      return Err("图片发送请求失败".to_string());
                    }
                }
        });
      }
    }
  }
  Ok(())
}

pub fn start_sensor_mpsc() -> Result<bool, String>{
    let (tx, rx) = std::sync::mpsc::sync_channel::<SensorsDataRequest>(32);
    // 存储 `tx` 在全局变量
    *GLOBAL_SENSOR_TX.lock().unwrap() = Some(tx);

    // 启动接收线程
    // 启动处理任务
    std::thread::spawn(move || {
      if let Err(e) = start_sensor_task(rx) {
          eprintln!("Sensor task error: {}", e);
      }
  });
  Ok(true)
}

// 通用消息队列(结果判定、数据返回前端、保存相关数据等)
pub enum GeneralRequest {
  // 启动机器人程序
  StartRobotProgram(oneshot::Sender<Result<(), String>>),
  // 零件到位启动机器人动作
  StartRobot(oneshot::Sender<Result<(), String>>),
  // 启动plc监控
  StartMonitorPLCProcess(oneshot::Sender<Result<(), String>>),
  // 启动机器人监控
  StartMonitorRobotProcess(oneshot::Sender<Result<(), String>>),
  // 保存图像结果到本地
  SaveImageResult(Vec<u8>, String, oneshot::Sender<Result<(), String>>), // 保存图像结果
  // 保存json到本地
  SaveJsonResult(String, String, oneshot::Sender<Result<(), String>>),  // 保存 JSON 结果
  // 向前端发送Log,显示在log框中
  SendLogToFrontend(String, oneshot::Sender<Result<(), String>>),  // 发送日志到前端
  // 向fastapi发送待处理图片
  SendImageToFastapi(Vec<u16>,cameras::hik_camera::FrameInfoSafe,Vec<u8>, oneshot::Sender<Result<(), String>>),
  // 发送图片到前端
  SendImageToFrontend(cameras::hik_camera::FrameInfoSafe,Vec<u8>, oneshot::Sender<Result<(), String>>),
  // 发送传感器数据到前端
  SendSensorDataToFrontend(Vec<u16>,f64, oneshot::Sender<Result<(), String>>),
  // 从plc获取当前状态
  GetCurrentState(oneshot::Sender<Result<String, String>>),  // 获取当前状态
  // 像前端发送json
  SendJsonToFrontend(String, oneshot::Sender<Result<(), String>>), //发送结果到前端
  // 像前端发送当前型号
  SendCurrentTypeToFrontend(oneshot::Sender<Result<(), String>>), //发送结果到前端
}

pub async fn start_global_task(mut rx: mpsc::Receiver<GeneralRequest>,app_handle: tauri::AppHandle) -> Result<(), String> {
  while let Some(request) = rx.recv().await {
    match request {
      GeneralRequest::StartRobotProgram(resp_tx) => {
        // 启动异步任务
        tokio::spawn(async {
            //机器人报警复位
            alarm_reset().await;
            // 等待200ms
            thread::sleep(Duration::from_millis(200));
            let log = "[robot] [log] [机器人报警复位<<<--]";
            sendlog2frontend(log.to_string());
            // 机器人上电
            on_battery().await;
            let log = "[robot] [log] [机器人上电<<<--]";
            sendlog2frontend(log.to_string());
            // 等待200ms
            thread::sleep(Duration::from_millis(1000));
            // 机器人主程序选择
            write_register_robot(4, 3).await;
            select_robot_program().await;
            let log = "[robot] [log] [机器人主程序选择]";
            sendlog2frontend(log.to_string());
            // 等待200ms
            thread::sleep(Duration::from_millis(200));
            // 启动机器人程序
            let result = start_robot_program().await;
            let log = "[robot] [log] [机器人主程序启动<<<--]";
            sendlog2frontend(log.to_string());
            thread::sleep(Duration::from_millis(200));
            write_register_robot(0, 0).await;

            let _ = resp_tx.send(result);
        });
      }
      GeneralRequest::StartRobot(resp_tx) => {
        // 启动异步任务
        tokio::spawn(async {
            let result = start_robot_process().await;
            let _ = resp_tx.send(result);
        });
      }
      GeneralRequest::StartMonitorPLCProcess(resp_tx) => {
        // 启动异步任务
        tokio::spawn(async {
            let result = monitor_plc().await;
            let _ = resp_tx.send(result);
        });
      }
      GeneralRequest::StartMonitorRobotProcess(resp_tx) => {
        // 启动异步任务
        tokio::spawn(async {
            let result = monitor_robot().await;
            let _ = resp_tx.send(result);
        });
      }

      // 保存图像结果
      GeneralRequest::SaveImageResult(image_data, path, resp_tx) => {
        // 假设这里是保存图片的逻辑
        let result = save_image(image_data, path).await;
        let _ = resp_tx.send(result);
      }
      // 保存 JSON 结果
      GeneralRequest::SaveJsonResult(json_data, path, resp_tx) => {
        // 假设这里是保存 JSON 文件的逻辑
        let result = save_json(json_data, path).await;
        let _ = resp_tx.send(result);
      }
      // 发送日志到前端
      GeneralRequest::SendLogToFrontend(log_message, resp_tx) => {
        // 假设这里是发送日志到前端的逻辑
        send_log_to_frontend(app_handle.clone(),log_message).await;
        let _ = resp_tx.send(Ok(()));
      }
      // 发送图像到前端
      GeneralRequest::SendImageToFrontend(frame_info,image_data, resp_tx) => {
        // 假设这里是发送图像到前端的逻辑
        let result = send_image_to_frontend(app_handle.clone(), frame_info, image_data).await;

        match result {
          Ok(_) => {
              // 成功处理图像
              let _ = resp_tx.send(Ok(()));
          }
          Err(e) => {
              // 处理失败的情况，记录错误
              eprintln!("发送图像失败: {}", e);
              
              // 发送错误时，将 `&str` 转换为 `String`
              let _ = resp_tx.send(Err(e.to_string()));
          }
        }
      }
      GeneralRequest::SendSensorDataToFrontend(pos,data, resp_tx) => {
        let result = send_sensor_data_to_frontend(app_handle.clone(),  pos,data).await;
        
        match result {
          Ok(_) => {
              // 成功处理图像
              let _ = resp_tx.send(Ok(()));
          }
          Err(e) => {
              // 处理失败的情况，记录错误
              eprintln!("发送传感器数据失败: {}", e);
              
              // 发送错误时，将 `&str` 转换为 `String`
              let _ = resp_tx.send(Err(e.to_string()));
          }
        }
      }

      GeneralRequest::SendImageToFastapi(pos,frame_info,image_data, resp_tx) => {
        let result = send_image_to_fastapi(app_handle.clone(), pos,frame_info, image_data).await;
        
        match result {
          Ok(_) => {
              // 成功处理图像
              let _ = resp_tx.send(Ok(()));
          }
          Err(e) => {
              // 处理失败的情况，记录错误
              eprintln!("发送图像失败: {}", e);
              
              // 发送错误时，将 `&str` 转换为 `String`
              let _ = resp_tx.send(Err(e.to_string()));
          }
        }
      }
      // 获取当前状态
      GeneralRequest::GetCurrentState(resp_tx) => {
        // 获取当前状态的逻辑
        let current_state = "正在运行".to_string(); // 假设状态为"正在运行"
        let _ = resp_tx.send(Ok(current_state));
      }
      GeneralRequest::SendJsonToFrontend(result, resp_tx)=>{
        send_json_to_frontend(result).await;
        let _ = resp_tx.send(Ok(()));
      }
      GeneralRequest::SendCurrentTypeToFrontend(resp_tx)=>{
        send_current_type_to_frontend(app_handle.clone()).await;
        let _ = resp_tx.send(Ok(()));
      }
    }
  }
  Ok(()) // 任务完成
}

// 以下是一些假设的函数来模拟保存、处理等操作
async fn save_image(image_data: Vec<u8>, path: String) -> Result<(), String> {
  // 模拟保存图像文件的逻辑
  println!("保存图像到路径: {}", path);
  Ok(())
}

async fn save_json(json_data: String, path: String) -> Result<(), String> {
  // 模拟保存 JSON 文件的逻辑
  println!("保存 JSON 到路径: {}", path);
  Ok(())
}

async fn send_log_to_frontend(app_handle:tauri::AppHandle, log_message: String) {
  // 通过 Tauri 事件系统向前端发送日志信息
  app_handle.emit("log_received", log_message.clone()).unwrap();
  println!("发送日志到前端: {}", log_message);
}

async fn send_image_to_frontend(
  app_handle:tauri::AppHandle,
  frame_info:cameras::hik_camera::FrameInfoSafe,
  image_data: Vec<u8>)-> Result<(), &'static str> {
  let result = read_multiple_registers_robot(256, 3).await;
  // 模拟发送图像到前端
  println!("发送图像到前端，图像大小: {} bytes", image_data.len());

  // bayerGB到RGB转换

  let width = frame_info.nWidth as i32;
  let height = frame_info.nHeight as i32;
  let mut mat = unsafe {
    Mat::new_rows_cols(height, width, CV_8U)
    .map_err(|_| "Mat 创建失败")?
  };
        
  // 获取 Mat 数据指针
  let mat_ptr = mat.data_mut();
  if mat_ptr.is_null() {
    return Err("Mat 数据指针为空");
  }

  // 复制 buffer 数据到 Mat
  unsafe {
      std::ptr::copy_nonoverlapping(image_data.as_ptr(), mat_ptr, image_data.len());
  }

  // 创建一个空 Mat 用于存放 RGB 图像数据
  let mut rgb_mat = Mat::new_rows_cols_with_default(
    height,
    width,
    CV_8UC3,
    Scalar::all(0.0),
  ).map_err(|_| "RGB Mat 创建失败")?;
        
  // 将 Bayer 格式转换为 RGB
  imgproc::cvt_color(&mat, &mut rgb_mat, imgproc::COLOR_BayerGB2RGB, 0,AlgorithmHint::ALGO_HINT_DEFAULT)
    .map_err(|_| "Bayer 到 RGB 转换失败")?;

  // 将 RGB 图像编码为 JPEG 格式
  let mut jpeg_data: Vec<u8> = Vec::new();
  let mut opencv_vector = Vector::new();
  opencv_vector.extend(jpeg_data.iter().cloned());
  imgcodecs::imencode(".jpg", &rgb_mat, &mut opencv_vector, &opencv::core::Vector::new())
      .map_err(|_| "JPEG 编码失败")?;

  // 将 JPEG 数据编码为 base64 格式
  let base64_image = base64::encode(&opencv_vector);
  //  获取当前位置信息

  // let result = read_multiple_registers_robot(256, 3).await;
  let mut reciever = String::from("image-send-image-1");
  match result {
    Ok(values) => {
        if values.len() == 3 {
            // 检查最后一个值，并执行相应的操作
            match values.last() {
                Some(&3) => {
                }
                Some(&4) => {
                    reciever = String::from("image-send-image-2");
                }
                _ => {
                    // 触发错误，抛弃
                }
            }
        } else {
            // 如果返回值不为 3 个元素，表示出错
            return Err("读取寄存器失败，返回的数据不足"); // 修改为 &'static str
        }
    }
    Err(_) => return Err("读取寄存器失败"), // 修改为 &'static str
  }
  // 发送rgb_mat到前端
  app_handle.emit(&reciever, base64_image).unwrap(); // 发送原始二进制数据到前端
  // 编码为jpg


  // 发送到前端
  Ok(())
}

async fn send_sensor_data_to_frontend(  
  app_handle:tauri::AppHandle,
  pos:Vec<u16>,
  data:f64,
)-> Result<(), &'static str>{
  // let result = read_multiple_registers_robot(256, 3).await;
  let mut reciever = String::from("sensor-send-data-1");
  match pos.last() {
    Some(&3) => {sendlog2frontend("[robot] [error] [传感器错误触发3]".to_string());}
    Some(&4) => {
        sendlog2frontend("[robot] [error] [传感器错误触发4]".to_string());
    }
    Some(&1) => {
      let mut task_state = GLOBAL_TASK_STATE.write().await;
      task_state.current_face = pos[0];
      task_state.current_hole = pos[1];
      task_state.add_hole(pos[0], pos[1]);
      if -40.0<data && data<40.0 {
        task_state.update_action1(pos[0],pos[1],data);
        }
      sendlog2frontend("[robot] [info] [传感器触发-1左侧]".to_string());
    }
    Some(&2) => {
      let mut task_state = GLOBAL_TASK_STATE.write().await;
      task_state.current_face = pos[0];
      task_state.current_hole = pos[1];

      if -40.0<data && data<40.0 {
        task_state.update_action2(pos[0],pos[1],data);
        }

      sendlog2frontend("[robot] [info] [传感器触发-2右侧]".to_string());
      reciever = "sensor-send-data-2".to_string();
    }
    _ => {
        println!("无效的机器人位置数据: {:?}", pos);
    }
  }

  if data > 100.0 || data < -100.0 {
    //  获取当前位置信息
    app_handle.emit(&reciever, "---").unwrap();
  } else {
      // 保证 data 总是保留 5 位有效数字
      let formatted_data = format!("{:.*}", 5, data);
      app_handle.emit(&reciever, formatted_data).unwrap();
  }
  Ok(())
}


async fn send_image_to_fastapi(
  app_handle:tauri::AppHandle,
  pos:Vec<u16>,
  frame_info:cameras::hik_camera::FrameInfoSafe,
  image_data: Vec<u8>)-> Result<(), &'static str>
{
  // bayerGB到RGB转换
  // let result = pos;
  let width = frame_info.nWidth as i32;
  let height = frame_info.nHeight as i32;
  let mut mat = unsafe {
    Mat::new_rows_cols(height, width, CV_8U)
    .map_err(|_| "Mat 创建失败")?
  };
        
  // 获取 Mat 数据指针
  let mat_ptr = mat.data_mut();
  if mat_ptr.is_null() {
    return Err("Mat 数据指针为空");
  }

  // 复制 buffer 数据到 Mat
  unsafe {
      std::ptr::copy_nonoverlapping(image_data.as_ptr(), mat_ptr, image_data.len());
  }

  // 创建一个空 Mat 用于存放 RGB 图像数据
  let mut rgb_mat = Mat::new_rows_cols_with_default(
    height,
    width,
    CV_8UC3,
    Scalar::all(0.0),
  ).map_err(|_| "RGB Mat 创建失败")?;
        
  // 将 Bayer 格式转换为 RGB
  imgproc::cvt_color(&mat, &mut rgb_mat, imgproc::COLOR_BayerGB2RGB, 0,AlgorithmHint::ALGO_HINT_DEFAULT)
    .map_err(|_| "Bayer 到 RGB 转换失败")?;

  let mut jpeg_data: Vec<u8> = Vec::new();
  let mut opencv_vector = Vector::new();
  opencv_vector.extend(jpeg_data.iter().cloned());
  imgcodecs::imencode(".jpg", &rgb_mat, &mut opencv_vector, &opencv::core::Vector::new())
      .map_err(|_| "JPEG 编码失败")?;


  let client = get_client().await;
  let part = Part::bytes(opencv_vector.to_vec())
      .file_name("image.jpg")
      .mime_str("image/jpeg")
      .map_err(|_| "构造 Part 失败")?;
  let form = Form::new().part("file", part);
  

  let mut fastapi_request = String::from("http://localhost:8000/detect_luowen_with_draw/");
  let mut reciever = String::from("image-send-image-1");
  // let result = read_multiple_registers_robot(256, 3).await;
  

  match pos.last() {
    Some(&3) => {sendlog2frontend("[robot] [log] [相机触发3 -左侧]".to_string());}
    Some(&4) => {
        sendlog2frontend("[robot] [log] [相机触发4 -右侧]".to_string());
        reciever = "image-send-image-2".to_string();
    }
    Some(&1) => {sendlog2frontend("[robot] [log] [相机触发-1]".to_string());}
    Some(&2) => {sendlog2frontend("[robot] [log] [相机触发-2]".to_string());}
    _ => {
        println!("无效的机器人位置数据: {:?}", pos);
    }
  }

  let response = client
      .post(&fastapi_request)
      .timeout(Duration::from_secs(1))
      .multipart(form)
      .send()
      .await
      .map_err(|_| "发送请求失败")?;
  
  // 5. 解析响应
  let response_json = response
      .json::<serde_json::Value>()
      .await
      .map_err(|_| "解析 JSON 失败")?;

  let results = response_json.get("results").unwrap_or(&serde_json::json!({})).clone();
  let image_base64 = response_json
      .get("image_base64")
      .and_then(|v| v.as_str())
      .unwrap_or("")
      .to_string();

  println!("收到 FastAPI 返回的数据: {:?}", results);

  // 6. 发送结果到前端
  //  获取当前位置信息

  app_handle.emit(&reciever, image_base64)
      .map_err(|_| "发送到前端失败")?;

  Ok(())
}


async fn send_json_to_frontend(result: String) {
  // 模拟发送图像到前端
  println!("发送图像到前端，图像大小: {} bytes", result);
}

async fn send_current_type_to_frontend(
  app_handle:tauri::AppHandle,
){
  // 读取配置中当前类型
  let current_type = get_current_type().clone();
  // 发送类型到前端
  app_handle.emit("current-type", current_type).unwrap();
}

// =========================================== FASTapi相关 ================================
static HTTP_CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

async fn get_client() -> Arc<Client> {
    HTTP_CLIENT
        .get_or_init(|| async { Arc::new(Client::new()) })
        .await
        .clone()
}
// ========================================================================================

// =========================================== 机器人相关 ==================================
// 机器人上电流程状态
#[derive(Debug, PartialEq,Copy, Clone)]
pub enum ResetState{
  ON,
  OFF,
}

lazy_static::lazy_static! {
  pub static ref ON_ROBOT_RESET: Arc<Mutex<ResetState>> = Arc::new(Mutex::new(ResetState::OFF));
}

#[tauri::command]
async fn start_robot_connect_frontend(robot_addr: String) -> Result<bool, String> {
  let robot_addr: std::net::SocketAddr = match robot_addr.parse() {
    Ok(addr) => addr,
    Err(_) => {
      return Err("无效的PLC地址".to_string());  // 如果解析失败，返回错误
  }
  };

  match start_robot_connect(robot_addr).await {
    Ok(true) => {
        println!("PLC 连接成功");
        Ok(true)  // 返回 true
    }
    Ok(false) => {
        println!("PLC 连接失败");
        Ok(false)  // 返回 false
    }
    Err(e) => {
        println!("PLC 连接发生错误: {}", e);
        Err(e)  // 发生错误，返回 false
    }
  }
}

#[tauri::command(rename_all = "snake_case")]
async fn read_register_frontend_robot(reg_address: u16) -> Result<u16, String> {
  read_register_robot(reg_address).await
}

#[tauri::command(rename_all = "snake_case")]
async fn write_register_frontend_robot(reg_address: u16, value: u16) -> Result<String, String> {
    // 调用原本的 write_register 函数
    match write_register_robot(reg_address, value).await {
      Ok(()) => Ok("success".to_string()),  // 成功返回 success
      Err(_) => Err("failed".to_string()),  // 失败返回 failed
  }
}
// 机器人监控信号
async fn monitor_robot() -> Result<(), String> {
  let log = "[robot] [log] [开启机器人监控]";
  sendlog2frontend(log.to_string());
  end_robot_process().await;
  loop {
      // 每500ms读取某个PLC寄存器，模拟读取过程
      tokio::time::sleep(std::time::Duration::from_millis(500)).await;
      let dev_state = {
        let lock = HARDWARE_DEV_STATE.lock().await; // 获取锁
        *lock // 复制出来，避免持有锁
      };
      if dev_state == HardwareDevState::ON{
        continue
      }else{
        // 获取plc到位信号
        match get_finished_from_robot().await{
          Ok(value) => {
            let state = {
              let lock = START_PROCESS_STATE.lock().await; // 获取锁
              *lock // 复制出来，避免持有锁
            };

            if value != 0 && state == SoftwareState::START{
              let log = "[robot] [log] [工件检测结束-->>>]";
              sendlog2frontend(log.to_string());
              // 向plc发送结束信号
              write_register_plc(7201, 0).await;
              send_robot_finished_to_plc().await;
              end_robot_process().await;

              let log = "[plc] [log] [工件退出<<<--]";
              sendlog2frontend(log.to_string());
              // 清除结果存储
              let mut task_state = GLOBAL_TASK_STATE.write().await;
              task_state.clear().await;
              // task_state.current_artifact = Local::now().format("%Y_%m_%d_%H_%M_%S_%3f").to_string();
              // task_state.current_artifact_type = current_type.clone().expect("current_type should not be None");

              let mut lock = START_PROCESS_STATE.lock().await; // 获取锁
              *lock = SoftwareState::STOP; // 设置为 START

            }else{
              // println!("等待工件到位");
            }
          }
          Err(err) => {
            // let log = "[robot] [error] [无法读取工件结束信息]";
            // sendlog2frontend(log.to_string());
          }
        }

        // 获取机器人指令信号
        match get_command_from_robot().await {
          Ok(value)=>{
            if value & 1 != 0 {
              // "[plc] [log] [机器人运行中<<<--]";
              write_register_robot(0, 0).await;
              send_continue_command_finished_to_plc().await;
              
              match get_command_from_plc().await {
                Ok(value)=>{
                  match value {
                    3 => {
                      write_register_plc(7202,0).await;
                    }
                    _ => {
                        // 
                    }
                  }
                }
                Err(err) => {
                  // let log = "[plc] [error] [无法读取工件指令信息]";
                  // sendlog2frontend(log.to_string());
                  }
              }
            }
            if value & 2 != 0 {
                let log = "[plc] [log] [机器人暂停中<<<--]";
                sendlog2frontend(log.to_string());
                send_pause_command_finished_to_plc().await;

                match get_command_from_plc().await {
                  Ok(value)=>{
                    match value {
                      2 => {
                        write_register_plc(7202,0).await;
                      }
                      
                      _ => {
                        // 
                      }
        
                    }
                  }
                  Err(err) => {
                    // let log = "[plc] [error] [无法读取工件指令信息]";
                    // sendlog2frontend(log.to_string());
                    }
                }
            }
            if value & 4 != 0 {
                // "[robot] [log] [机器人伺服上电完成<<<--]";

            }
            if value & 8 != 0 {
              let  log = "[robot] [error] [机器人报警故障-->>>]";
              sendlog2frontend(log.to_string());
              // 机器人故障发送给plc停止
              // send_reset_command_finished_to_plc().await;
              send_robot_err_to_plc().await;
            }
            if value & 16 != 0 {
                // "[robot] [log] [机器人报警复位完成<<<--]";
                let mut lock = ON_ROBOT_RESET.lock().await; // 获取锁
                *lock = ResetState::OFF;
                send_reset_command_finished_to_plc().await;

                match get_command_from_plc().await {
                  Ok(value)=>{
                    match value {
                      4 => {
                        write_register_plc(7202,0).await;
                      }
                      
                      _ => {
                          // 
                      }
        
                    }
                  }
                  Err(err) => {
                    // let log = "[plc] [error] [无法读取工件指令信息]";
                    // sendlog2frontend(log.to_string());
                    }
                }
            }
            if value & 32 == 0 {
              // "[robot] [log] [机器人急停中<<<--]";
              send_robot_err_to_plc().await;
          }
          }
          Err(err) => {
            // let log = "[robot] [error] [无法读取工件指令信息]";
            // sendlog2frontend(log.to_string());
            }
        }
      }
  }
}

// 手动启动机器人
#[tauri::command(rename_all = "snake_case")]
async fn reset_start_robot() -> Result<(), String> {
  tauri::async_runtime::spawn(async {
    let (resp_tx, resp_rx) = oneshot::channel(); 
    let tx = GLOBAL_TX.lock().await.clone().unwrap_or_else(|| {
      panic!("GLOBAL_TX is not initialized. Ensure that start_global_task() has been called.");
    });
    tx.send(GeneralRequest::StartRobotProgram(resp_tx)).await.map_err(|_| "启动机器人程序失败".to_string());
  });

  Ok(())
}

// 手动报警复位机器人
#[tauri::command(rename_all = "snake_case")]
async fn reset_alarm_robot() -> Result<(), String> {
  tauri::async_runtime::spawn(async {
    alarm_reset().await;
    thread::sleep(Duration::from_millis(200));
    write_register_robot(0, 0).await;

  });

  Ok(())
}

// 报警复位
async fn alarm_reset() -> Result<(), String> {
  // 保证暂停和停止信号常开
  write_register_robot(4, 3).await;
  write_register_robot(0, 8).await
}


// 手动暂停机器人
#[tauri::command(rename_all = "snake_case")]
async fn stop_robot() -> Result<(), String> {
  tauri::async_runtime::spawn(async {
    write_register_robot(0, 0).await;
    thread::sleep(Duration::from_millis(200));
    write_register_robot(4, 1).await;

  });

  Ok(())
}

// 暂停
async fn pause_robot() -> Result<(), String>{
  write_register_robot(0, 0).await;
  write_register_robot(4, 1).await
}


// 手动继续机器人
#[tauri::command(rename_all = "snake_case")]
async fn start_robot() -> Result<(), String> {
  tauri::async_runtime::spawn(async {
    write_register_robot(4, 3).await;
    write_register_robot(0, 1).await

  });
  Ok(())
}

// 启动机器人程序
async fn start_robot_program() -> Result<(), String> {
  // 保证暂停和停止信号常开
  write_register_robot(4, 3).await;
  write_register_robot(0, 1).await
}

// 手动到位信号
#[tauri::command(rename_all = "snake_case")]
async fn position_robot() -> Result<(), String> {
  tauri::async_runtime::spawn(async {
    // 手动启动机器人
    write_register_robot(3, 1).await;
    thread::sleep(Duration::from_millis(1000));
    write_register_robot(3, 0).await

  });

  Ok(())
}


// 机器人上电
async fn on_battery() -> Result<(), String> {
  write_register_robot(0, 16).await
}
// 机器人主程序选择
async fn select_robot_program() -> Result<(), String> {
  write_register_robot(0, 64).await
}


// 从机器人获取指令结束信息
async fn get_command_from_robot()-> Result<u16, String>{
  read_register_robot(260).await
}

// 到位
async fn start_robot_process() -> Result<(), String>{
  write_register_robot(3, 1).await
}

// 关闭到位信号
async fn end_robot_process() -> Result<(), String>{
  write_register_robot(3, 0).await
}

// 读取当前位置，每次相机触发后读取
async fn get_current_pos_from_robot()-> Result<Vec<u16>, String>{

  let result = read_multiple_registers_robot(256, 3).await;
    
  match result {
      Ok(values) => {
          if values.len() == 3 {
              // 返回读取到的值
              Ok(values)
          } else {
              // 如果返回值不为 3 个元素，表示出错
              Err("读取寄存器失败，返回的数据不足".to_string())
          }
      }
      Err(err) => Err(err),  // 如果读取失败，返回错误
  }
}

// 动作完成读取，每次相机触发后读取
async fn get_finished_from_robot()-> Result<u16, String>{
  read_register_robot(259).await

}

// ========================================================================================

// =========================================== 算法相关 ====================================
// 算法相关代码位于sidecar模块内

// 访问当前选择类型
#[tauri::command(rename_all = "snake_case")]
async fn toggle_diameter_on() -> Result<(), String>  {
  // let mut lock = HARDWARE_DEV_STATE.lock().await; // 获取锁
  // *lock = HardwareDevState::ON;

  Ok(())
}
// 访问当前选择类型
#[tauri::command(rename_all = "snake_case")]
async fn toggle_depth_on() -> Result<(), String>  {
  // let mut lock = HARDWARE_DEV_STATE.lock().await; // 获取锁
  // *lock = HardwareDevState::ON;

  Ok(())
}

// 访问当前选择类型
#[tauri::command(rename_all = "snake_case")]
async fn toggle_detect_on() -> Result<(), String>  {
  // let mut lock = HARDWARE_DEV_STATE.lock().await; // 获取锁
  // *lock = HardwareDevState::ON;

  Ok(())
}

// 访问当前选择类型
#[tauri::command(rename_all = "snake_case")]
async fn toggle_save_image_on() -> Result<(), String>  {
  // let mut lock = HARDWARE_DEV_STATE.lock().await; // 获取锁
  // *lock = HardwareDevState::ON;

  Ok(())
}
// ========================================================================================

// =========================================== plc相关 =====================================

#[tauri::command]
async fn start_plc_connect_frontend(plc_addr: String) -> Result<bool, String> {
  let plc_addr: std::net::SocketAddr = match plc_addr.parse() {
    Ok(addr) => addr,
    Err(_) => {
      return Err("无效的PLC地址".to_string());  // 如果解析失败，返回错误
  }
  };

  match start_plc_connect(plc_addr).await {
    Ok(true) => {
        println!("PLC 连接成功");
        Ok(true)  // 返回 true
    }
    Ok(false) => {
        println!("PLC 连接失败");
        Ok(false)  // 返回 false
    }
    Err(e) => {
        println!("PLC 连接发生错误: {}", e);
        Err(e)  // 发生错误，返回 false
    }
  }
}

#[tauri::command(rename_all = "snake_case")]
async fn write_register_frontend_plc(reg_address: u16, value: u16) -> Result<String, String> {
    // 调用原本的 write_register 函数
    match write_register_plc(reg_address, value).await {
      Ok(()) => Ok("success".to_string()),  // 成功返回 success
      Err(_) => Err("failed".to_string()),  // 失败返回 failed
  }
}


#[tauri::command(rename_all = "snake_case")]
async fn read_register_frontend_plc(reg_address: u16) -> Result<u16, String> {
    read_register_plc(reg_address).await
}

// 手动发送机器人运行信号
#[tauri::command(rename_all = "snake_case")]
async fn robot_running() -> Result<(), String> {
  send_continue_command_finished_to_plc().await;
  Ok(())
}

// 手动发送机器人暂停中信号
#[tauri::command(rename_all = "snake_case")]
async fn robot_pausing() -> Result<(), String> {
  send_pause_command_finished_to_plc().await;
  Ok(())
}

// 手动发送机器人复位完成信号
#[tauri::command(rename_all = "snake_case")]
async fn robot_reset_finished() -> Result<(), String> {
  send_reset_command_finished_to_plc().await;
  Ok(())
}

// 手动发送机器人拍照完成信号
#[tauri::command(rename_all = "snake_case")]
async fn robot_run_finished() -> Result<(), String> {
  write_register_plc(7201, 0).await;
  send_robot_finished_to_plc().await;
  end_robot_process().await;

  Ok(())
}


// 启动plc监控过程的异步任务
async fn monitor_plc() -> Result<(), String> {
  let log = "[plc] [log] [开启PLC监控]";
  sendlog2frontend(log.to_string());
  loop {
      // 每500ms读取某个PLC寄存器，模拟读取过程
      tokio::time::sleep(std::time::Duration::from_millis(500)).await;

      // 机器人是否处于运行中
      let state = {
        let lock = START_PROCESS_STATE.lock().await; 
        *lock 
      };

      // 硬件DEV模式
      let dev_state = {
          let lock = HARDWARE_DEV_STATE.lock().await; 
          *lock 
      };

      if dev_state == HardwareDevState::ON{
        // continue
      }else{
        if state ==SoftwareState::STOP {
          let current_type = get_current_type().clone();

          match get_type_from_plc().await{
            Ok(value)=>{
              let plc_type = map_value_to_type_plc(value);

              // 写入类型到机器人
              match (plc_type, current_type.clone()) {
                (Some(plc_type_str), Some(current_type_str)) => {
                    if plc_type_str != current_type_str {
                        // 如果不同，调用 set_current_type 更新类型

                        set_current_type(plc_type_str); // 调用异步函数更新型号
                        let log = "[plc] [log] [修改机器人型号-->>>]";
                        sendlog2frontend(log.to_string());
                        // 写入类型到机器人

                        let new_type = get_current_type();
                        match new_type {
                          Some(robot_type) => {
                              println!("当前型号: {}", robot_type);
          
                              // 将型号写入机器人
                              match write_current_type_to_robot(&robot_type).await {
                                  Ok(_) => {
                                      let log = "[robot] [log] [写入机器人型号<<<--]";
                                      sendlog2frontend(log.to_string());
                                      // 继续执行，不需要返回 Err
                                  }
                                  Err(err) => {
                                      println!("写入机器人型号失败: {}", err);
                                      return Err(err); // 明确返回错误
                                  }
                              }

                              tauri::async_runtime::spawn(async {
                                let (resp_tx, resp_rx) = oneshot::channel(); 
                                let tx = GLOBAL_TX.lock().await.clone().unwrap_or_else(|| {
                                  panic!("GLOBAL_TX is not initialized. Ensure that start_global_task() has been called.");
                                });
                                tx.send(GeneralRequest::SendCurrentTypeToFrontend(resp_tx)).await.map_err(|_| "发送型号失败".to_string());
                              });
                          }
                          None => {
                              println!("无法获取当前型号");
                              return Err("无法获取当前型号".to_string()); // 明确返回错误
                          }
                        }
                    } else {
                        // println!("当前类型与PLC获取的类型一致，无需更新");
                    }
                }
                _ => {
                    println!("无法获取类型进行比较");
                }
              }
            }
            Err(err) => {
              // let log = "[plc] [error] [无法读取工件型号信息]";
              // sendlog2frontend(log.to_string());
            }
          }
          match get_start_robot_from_plc_started().await{
            Ok(value) => {
              if value != 0 {

                // 修改状态为START
                let mut lock = START_PROCESS_STATE.lock().await; // 获取锁
                *lock = SoftwareState::START; // 设置为 START

                match current_type.clone() {
                  Some(robot_type) => {

                      // 将型号写入机器人
                      match write_current_type_to_robot(&robot_type).await {
                          Ok(_) => {
                            // let log = format!("[robot] [log] [写入型号：{} <<<--]",robot_type);
                            // sendlog2frontend(log.to_string());
                          }
                          Err(err) => {
                            // println!("写入机器人型号失败: {}", err);
                            return Err(err); // 明确返回错误
                          }
                      }
                  }
                  None => {
                      println!("无法获取当前型号");
                      return Err("无法获取当前型号".to_string()); // 明确返回错误
                  }
                }
                // 创建保存状态
                let mut task_state = GLOBAL_TASK_STATE.write().await;
                task_state.current_artifact = Local::now().format("%Y_%m_%d_%H_%M_%S_%3f").to_string();
                task_state.current_artifact_type = current_type.clone().expect("current_type should not be None");
                // 发送到位信号到机器人
                start_robot_process().await;
              }else{
                // println!("等待工件到位或上次过程完成");
              }
            }
            Err(err) => {
                  // let log = "[plc] [error] [无法读取工件位置信息]";
                  // sendlog2frontend(log.to_string());
                  }
          }
        }
        match get_command_from_plc().await {
          Ok(value)=>{
            match value {
              2 => {
                  // 如果 value 是 2，给机器人暂停信号
                  let log = "[plc] [log] [机器人暂停-->>>]";
                  sendlog2frontend(log.to_string());
                  pause_robot().await;
                  let log = "[robot] [log] [机器人暂停<<<--]";
                  sendlog2frontend(log.to_string());
                  // 在这里执行针对 value == 2 的操作
              }
              3 => {
                  // 如果 value 是 3，给机器人继续信号
                  let log = "[plc] [log] [机器人继续-->>>]";
                  sendlog2frontend(log.to_string());
                  start_robot_program().await;
                  let log = "[robot] [log] [机器人继续<<<--]";
                  sendlog2frontend(log.to_string());
                  // 在这里执行针对 value == 3 的操作
              }
              4 => {
                  // 如果 value 是 4，给机器人复位信号
                  let log = "[plc] [log] [机器人复位-->>>]";
                  sendlog2frontend(log.to_string());
                  // alarm_reset().await;
                  let state = {
                    let lock = ON_ROBOT_RESET.lock().await; // 获取锁
                    *lock // 复制出来，避免持有锁
                  };
                  if state == ResetState::OFF {
                    
                    let mut lock = ON_ROBOT_RESET.lock().await; // 获取锁
                    *lock = ResetState::ON;

                    tauri::async_runtime::spawn(async {
                      let (resp_tx, resp_rx) = oneshot::channel(); 
                      let tx = GLOBAL_TX.lock().await.clone().unwrap_or_else(|| {
                        panic!("GLOBAL_TX is not initialized. Ensure that start_global_task() has been called.");
                      });
                      tx.send(GeneralRequest::StartRobotProgram(resp_tx)).await.map_err(|_| "启动机器人程序失败".to_string());
                    });
                    let log = "[robot] [log] [机器人复位<<<--]";
                  sendlog2frontend(log.to_string());
                  }else{

                  }

                  let log = "[robot] [log] [机器人复位<<<--]";
                  sendlog2frontend(log.to_string());
                  // 等待200ms
                  thread::sleep(Duration::from_millis(200));
                  write_register_robot(0, 0).await;
                  
              }
              _ => {
                  // 
              }

            }
          }
          Err(err) => {
            // let log = "[plc] [error] [无法读取工件指令信息]";
            // sendlog2frontend(log.to_string());
            }
        }
    }
  }
}

async fn get_type_from_plc()-> Result<u16, String>{
  read_register_plc(1000).await
}

async fn get_start_robot_from_plc_started() -> Result<u16, String>{
  read_register_plc(7201).await

}

// 流程结束后写入
async fn send_robot_finished_to_plc(){
  write_register_plc(7301, 1).await;
}


// plc到机器人的中转信号
async fn get_command_from_plc()-> Result<u16, String>{
  read_register_plc(7202).await
}


// 中转完成信号-暂停
async fn send_pause_command_finished_to_plc(){
  write_register_plc(7302, 2).await;
}

// 中转完成信号-继续
async fn send_continue_command_finished_to_plc(){
  write_register_plc(7302, 3).await;
}

// 中转完成信号-复位
async fn send_reset_command_finished_to_plc(){
  write_register_plc(7302, 4).await;
}

// 机器人急停信号
async fn send_robot_err_to_plc(){
  write_register_plc(7302, 5).await;
}

// =========================================================================================

// =========================================== 其他 ========================================
// 前端类型选择
#[tauri::command(rename_all = "snake_case")]
async fn selected_artifact_type(){
    tauri::async_runtime::spawn(async {
    let (resp_tx, resp_rx) = oneshot::channel(); 
    let tx = GLOBAL_TX.lock().await.clone().unwrap_or_else(|| {
      panic!("GLOBAL_TX is not initialized. Ensure that start_global_task() has been called.");
    });
    tx.send(GeneralRequest::SendCurrentTypeToFrontend(resp_tx)).await.map_err(|_| "启动机器人程序失败".to_string());
  });
}

// 访问当前选择类型
#[tauri::command]
async fn frontend_select_artifact_type(artifactType: String){
  // 写入型号到plc
  write_current_type_to_plc(&artifactType).await.unwrap_or_else(|err| {
    let log = format!("[software] [error] [型号写入错误：{}]", artifactType);
    sendlog2frontend(log.to_string());
  });
  let log = format!("[software] [info] [修改型号：{}]", artifactType);
  sendlog2frontend(log.to_string());
}

// 访问当前选择类型
#[tauri::command(rename_all = "snake_case")]
async fn toggle_hardware_dev_on() -> Result<(), String>  {
  let mut lock = HARDWARE_DEV_STATE.lock().await; // 获取锁
  *lock = HardwareDevState::ON;

  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn toggle_hardware_dev_off() -> Result<(), String>  {
  let mut lock = HARDWARE_DEV_STATE.lock().await; // 获取锁
  *lock = HardwareDevState::OFF;
  Ok(())
}

// 获取软件型号配置
pub fn get_current_type()->Option<String>{
  let config = match crate::config::config::CONFIG.read() {
    Ok(config) => config,
    Err(_) => {
        println!("获取配置失败");
        return None;
    }
  };
  // 检查配置是否已经加载
  if let Some(config) = &*config {
    // 访问硬件配置中的 plc 子配置项
    if let Some(current_type) = config.recipes.get_value("type.current_type") {
        if let Some(current_type_) = current_type.as_str(){
            // 返回ip_port
            return Some(current_type_.to_string());
          } else {
            println!("工件型号 不是字符串类型");
          }
    } else {
        println!("工件型号不存在.");
    }
  } else {
    println!("配置加载失败");
  }
  None  // 如果找不到，返回 None
}

// 设置软件当前零件型号
pub fn set_current_type(new_type: String)->Result<(), Box<dyn std::error::Error>>{
  let mut config = match crate::config::config::CONFIG.write() {
    Ok(config) => config,
    Err(_) => {
        println!("获取配置失败");
        return Err("获取配置失败".into());
    }
  };
  // 检查配置是否已经加载
  if let Some(config) = &mut *config {
    // 调用 set_value 更新配置中的 "type.current_type" 字段
    let new_value = Value::String(new_type.clone());
    
    // 使用 set_value 来更新配置文件
    config.recipes.set_value("type.current_type", new_value)?;

    println!("当前零件型号已更新为: {}", new_type);
    Ok(())
} else {
    println!("配置加载失败");
    Err("配置加载失败".into())
}
}

// 型号写入
// 将value（0-4）映射为相应的型号字符串
fn map_value_to_type_plc(value: u16) -> Option<String> {
  let type_map: HashMap<u16, &str> = vec![
      (0, "EH09"),
      (1, "EH12"),
      (2, "EK30"),
      (3, "EK40"),
      (4, "EY28"),
  ]
  .into_iter()
  .collect();

  type_map.get(&value).map(|&s| s.to_string())
}

// 写入型号到机器人
async fn write_current_type_to_robot(robot_type: &str) -> Result<(), String>{
  let value = match robot_type {
    "EH09" => 0,
    "EH12" => 1,
    "EK30" => 2,
    "EK40" => 3,
    "EY28" => 4,
    "TEST" => 10,
    _ => return Err("未知的机器人型号".to_string()),
  };
  write_register_robot(1, value).await
}

// 写入型号到plc
async fn write_current_type_to_plc(robot_type: &str) -> Result<(), String>{
  let value = match robot_type {
    "EH09" => 0,
    "EH12" => 1,
    "EK30" => 2,
    "EK40" => 3,
    "EY28" => 4,
    "TEST" => 10,
    _ => return Err("未知的机器人型号".to_string()),
  };
  write_register_plc(1000, value).await
}

// =========================================================================================

// =========================================== 整体逻辑 ====================================
// 启动或终止后端程序，暂时没用
#[tauri::command(rename_all = "snake_case")]
fn start_software(start_state: String) -> String {
    match start_state.as_str(){
      // 启动后端流程
      "start" => {
        // let mut state = START_STATE.lock().unwrap();
        // *state = SoftwareState::START;
        // 创建响应通道
        "started".to_string()
      },
      // 终止后端流程
      "end"   => {
        let mut state = START_STATE.lock().unwrap();
        *state = SoftwareState::STOP;
        
        "ended".to_string()
      },
        _      => {
        "wrong param".to_string()
      },
    }
}

// 整体逻辑
async fn start_global_mpsc(app_handle: tauri::AppHandle) -> Result<bool, String> {
  let (tx, rx) = mpsc::channel::<GeneralRequest>(32);

  let tx = Arc::new(Mutex::new(Some(tx))); // 用 Mutex 包装 tx
  // 将 tx 存储在全局变量中
  *GLOBAL_TX.lock().await = Some(tx.lock().await.clone().unwrap());

  // 启动异步任务
  tokio::spawn(start_global_task(rx,app_handle.clone()));
  Ok(true) // 成功返回 true

}

fn start_global_mpsc_(app_handle: tauri::AppHandle){
  tauri::async_runtime::spawn(async move{
    // 这里可以执行一些后台任务
    println!("主线程通道启动");
    // plc启动
    start_global_mpsc(app_handle.clone()).await;
    println!("主线程通道创建完毕");
  });
}

// ========================================================================================

// =========================================== 测试用 ======================================
// 前端简单调用测试
#[tauri::command]
fn test_call_from_frontend() -> String {
    println!("I was invoked from JavaScript!");
    "Call from frontend: test ok".to_string()
}

#[tauri::command]
fn test_image_transfer_to_frontend() -> String {
    println!("I was invoked from JavaScript!");
    "Call from frontend: test ok".to_string()
}

// fastapi启动成功测试
async fn test_fastapi_backend(app_handle:tauri::AppHandle){
  // 4. 发送 HTTP 测试请求到 FastAPI
  // println!("line 472");
  // let client = get_client().await;
  // let response = client
  //   .get("http://localhost:8000/")
  //   .timeout(Duration::from_secs(10))
  //   .send()
  //   .await;

  // 处理响应
  // match response {
  //   Ok(resp) => match resp.text().await {
  //       Ok(text) => println!("FastAPI 响应: {}", text),
  //       Err(err) => eprintln!("读取响应失败: {}", err),
  //   },
  //   Err(err) => eprintln!("请求失败: {}", err),
  // }
}

// ========================================================================================
pub fn sendlog2frontend(log:String)-> Result<(), String>{
  // 发送日志到前端的异步任务
  
  tokio::spawn(async move { 
    let (resp_tx, resp_rx) = oneshot::channel();
    let tx = GLOBAL_TX.lock().await.clone().unwrap_or_else(|| {
        panic!("GLOBAL_TX is not initialized. Ensure that start_plc_connect() has been called.");
    });
    // 在异步任务中处理发送日志

    tx.send(GeneralRequest::SendLogToFrontend(log, resp_tx))
        .await
        .map_err(|_| "发送请求失败".to_string());

    // 处理接收响应
    match resp_rx.await {
        Ok(_) => {
        }
        Err(e) => {
            println!("日志发送失败 {}！",e);
        }
    }
  });
  Ok(())
}
// ===========================================================================================

// ============================================== tauri相关 ==================================
fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {

  app.manage(Arc::new(std::sync::Mutex::new(None::<Arc<std::sync::Mutex<Child>>>)));

  // 加载全局配置
  let run_path = app.path().resolve("assets/config/run_settings.toml", BaseDirectory::Resource)?.to_path_buf();
  let recipes_path = app.path().resolve("assets/config/recipes_settings.toml", BaseDirectory::Resource)?.to_path_buf();
  let hardware_path = app.path().resolve("assets/config/hardware_settings.toml", BaseDirectory::Resource)?.to_path_buf();
  let algo_path = app.path().resolve("assets/config/algo_settings.toml", BaseDirectory::Resource)?.to_path_buf();

  let config_tmp = config::config::Config::load(run_path, recipes_path, hardware_path, algo_path)?;

  {
  let mut config = config::config::CONFIG.write().unwrap();  // 获取写入锁
  *config = Some(config_tmp.clone());  // 更新配置
  }
  // Clone the app handle for use elsewhere
  let app_handle = app.handle().clone();

  // 启动plc modbus tcp异步通道
  start_plc_connection();
  // 启动机器人 modbus tcp异步通道
  start_robot_connection();
  // 启动主通道
  start_global_mpsc_(app_handle.clone());
  // 启动传感器通道
  start_sensor_mpsc().expect("Failed to start sensor mpsc");

  // 启动相机
  init_mvs_sdk();
  // sensors::cf3000::main_();

  // 启动机器人异步通道
  tauri::async_runtime::spawn(async {
    // 启动PLC监控程序
    let (resp_tx, resp_rx) = oneshot::channel(); 
    let (resp_tx1, resp_rx1) = oneshot::channel(); 
    let (resp_tx2, resp_rx2) = oneshot::channel(); 
    let tx = GLOBAL_TX.lock().await.clone().unwrap_or_else(|| {
      panic!("GLOBAL_TX is not initialized. Ensure that start_global_task() has been called.");
    });
    tx.send(GeneralRequest::StartRobotProgram(resp_tx1)).await.map_err(|_| "启动机器人程序失败".to_string());
    tx.send(GeneralRequest::StartMonitorPLCProcess(resp_tx)).await.map_err(|_| "启动plc监控失败".to_string());
    tx.send(GeneralRequest::StartMonitorRobotProcess(resp_tx2)).await.map_err(|_| "启动机器人监控失败".to_string());
  });

  tauri::async_runtime::spawn(async move{
    enumerate_devices(MvEnumDeviceLayerType::GigeDevice).await;
    match create_handle().await{
      Ok(results) => {
        // 打印返回的结果 Vec<bool>
        if let Some(first_element) = results.get(0) {
          let log = format!("[camera] [log] [相机句柄: {}]", first_element);
          sendlog2frontend(log.to_string());
        } else {
            println!("Vec 为空");
        }
      }

      Err(e) => {
        // 处理错误，打印错误信息
        let log = format!("[camera] [error] [相机句柄创建失败: {}]", e);
        sendlog2frontend(log.to_string());
      }
    }

    match open_device(MvAccessMode::Exclusive, 0).await {
      Ok(code) => {
        println!("设备打开成功，返回码: {}", code);
        let log = format!("[camera] [log] [打开相机成功: {}]", code);
        sendlog2frontend(log.to_string());
      }
      Err(e) => {
        let log = format!("[camera] [error] [打开相机失败]");
        sendlog2frontend(log.to_string());
      }
    }

    let result = is_device_connected().await; // 存储返回值
    match result {
        Ok(connected) => {
            if connected {
                println!("设备已连接");
            } else {
                println!("设备未连接");
            }
        }
        Err(e) => {
            eprintln!("查询设备连接状态失败: {}", e);
        }
    }

    let result = register_callback().await; // 存储返回值
    match result {
        Ok(code) => println!("设备注册回调，返回码: {}", code),
        Err(e) => {
            eprintln!("回调注册失败: {}", e);
        }
    }

    match start_grabbing().await {
      Ok(code) => println!("开始取流成功，返回码: {}", code),
      Err(e) => eprintln!("开始取流失败: {}", e),
    }

  // 注册传感器回调
  rs_CF_RegisterEventCallback().await;
  let ip = "192.168.0.101";
  let mut device_handler: sensors::cf3000_bindings::DeviceHandle_t = -1;
  rs_CF_GE_OpenDevice(ip,&mut device_handler).await;
  let log = format!("[sensor] [log] [传感器: {}]", device_handler);
  sendlog2frontend(log.to_string());


  let cmd:bool = true;
  rs_CF_StartSample(device_handler,cmd).await;
  let log = format!("[sensor] [log] [启动传感器数据采集]");
  sendlog2frontend(log.to_string());

  });

  println!("前端窗口已加载，启动后台任务");
  println!("[tauri] Creating fastapi sidecar...");
  sidecar::sidecar::spawn_and_monitor_sidecar(app_handle.clone()).ok();
  println!("[tauri] Fastapi Sidecar spawned and monitoring started.");
  // 启动后端任务
  let mut state = START_STATE.lock().unwrap();
  *state = SoftwareState::START;

  

  Ok(())
}

static mut WINDOW_CLOSING: bool = false;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run_tauri_app() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .setup(setup)
    .on_window_event(move |window, event| match event {
      WindowEvent::CloseRequested { api, .. } => {
          println!("即将关闭窗口，等待清理资源...");
          unsafe {
            // 检查是否已经开始关闭窗口，避免重复操作
            if WINDOW_CLOSING {
                return;
            }
            WINDOW_CLOSING = true; // 标记窗口正在关闭
          }
          api.prevent_close(); // 防止窗口立即关闭

          let window = window.clone();
          tauri::async_runtime::spawn(async move {
              stop_plc_connection().await;
              println!("plc modbus 资源清理完成");
              stop_robot_connection().await;
              println!("机器人 modbus 资源清理完成");
              // 销毁相机句柄
              match stop_grabbing().await {
                  Ok(code) => println!("停止取流成功，返回码: {}", code),
                  Err(e) => eprintln!("停止取流失败: {}", e),
              }


                match close_device().await {
                    Ok(code) => println!("设备关闭成功，返回码: {}", code),
                    Err(e) => eprintln!("设备关闭失败: {}", e),
                }

                match destroy_handle().await {
                    Ok(_) => {
                        // 销毁设备句柄成功
                        println!("destroy handle successfully");
                    }
                    Err(e) => {
                        // 销毁设备句柄失败，处理错误
                        eprintln!("设备句柄销毁失败: {}", e);
                    }
                }

            // 关闭相机  
              window.close().unwrap();
          });
          
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![start_software,
                                            test_call_from_frontend,
                                            read_register_frontend_plc,
                                            read_register_frontend_robot,
                                            write_register_frontend_plc,
                                            write_register_frontend_robot,
                                            frontend_select_artifact_type,
                                            selected_artifact_type,
                                            // 机器人调试
                                            reset_start_robot,
                                            reset_alarm_robot,
                                            position_robot,
                                            start_robot,
                                            stop_robot,
                                            // plc调试
                                            robot_reset_finished,
                                            robot_pausing,
                                            robot_running,
                                            robot_run_finished,
                                            toggle_hardware_dev_on,
                                            toggle_hardware_dev_off,
                                            // 算法设置
                                            toggle_detect_on,
                                            toggle_depth_on,
                                            toggle_diameter_on,
                                            toggle_save_image_on,
                                            sidecar::sidecar::start_sidecar,
                                            sidecar::sidecar::shutdown_sidecar])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
// ========================================================================================


