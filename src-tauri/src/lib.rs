use std::sync::{Arc, RwLock};
use std::thread;
use std::fs::File;
use std::io::Read;
use tokio::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;
use tauri::{AppHandle, Manager,Emitter};

use std::time::Duration;
use tokio::process::Child; // 导入 tokio::process::Child
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use tauri::{State, WindowEvent};
use tokio::task;
mod sidecar;
mod plc;
use plc::modbusTCP;

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

use opencv::{
  core::{Mat, MatTrait, CV_8UC3, CV_8U,Scalar,AlgorithmHint},
  imgcodecs,
  imgproc,
  prelude::*,
};
use std::fs;

// 主流程启动状态
pub enum SoftwareState{
  START,
  STOP,
}

// 硬件连接状态
pub enum HardwareState{
  plc(bool),
  camera(bool),
  sensor(bool),
  robot(bool),
}

pub enum PLCProtocol{
  p_socket(u8),
  p_modbusTCP(u8),
}

lazy_static::lazy_static! {
  pub static ref START_STATE: Arc<std::sync::Mutex<SoftwareState>> = Arc::new(std::sync::Mutex::new(SoftwareState::STOP));
}


pub static mut PLC_STATE: HardwareState = HardwareState::plc(false);
pub static mut CAMERA_STATE: HardwareState = HardwareState::camera(false);
pub static mut SENSOR_STATE: HardwareState = HardwareState::sensor(false);
pub static mut ROBOT_STATE: HardwareState =HardwareState::robot(false);

// modbus连接
use lazy_static::lazy_static;
// modbusTCP消息队列
lazy_static! {
  static ref PLC_TX: Arc<Mutex<Option<mpsc::Sender<modbusTCP::ModbusRequest>>>> = Arc::new(Mutex::new(None));
}


lazy_static! {
  static ref GLOBAL_TX: Arc<Mutex<Option<mpsc::Sender<GeneralRequest>>>> = Arc::new(Mutex::new(None));
}

lazy_static! {
  static ref GLOBAL_SENSOR_TX: Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<SensorsDataRequest>>>> =
      Arc::new(std::sync::Mutex::new(None));
}


// 机器人消息队列(读写)

// 当前任务存放
pub struct TaskState {

}

pub struct HoleState {

}



// 数据消息队列(相机、传感器)
pub enum SensorsDataRequest {
  ImageProcess(cameras::hik_camera::FrameInfoSafe,Vec<u8>),
  Cf3000(Vec<u8>),
}

// 初始化传感器消息队列
pub fn start_sensor_task(mut rx: std::sync::mpsc::Receiver<SensorsDataRequest>) -> Result<(), String> {
  let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?; // 创建一个 Tokio 运行时

  while let Ok(request) = rx.recv() {
    match request {
      SensorsDataRequest::ImageProcess(frame_info,image_data)=>{
        let log = "[camera] [log] [info]";
        // rt.spawn(async move {
        //     let (resp_tx, resp_rx) = oneshot::channel();
        //     let tx = GLOBAL_TX.lock().await.clone().unwrap_or_else(|| {
        //         panic!("GLOBAL_TX is not initialized. Ensure that start_plc_connect() has been called.");
        //     });
        //     // 在异步任务中处理发送日志
        
        //     tx.send(GeneralRequest::SendLogToFrontend(log.to_string(), resp_tx))
        //         .await
        //         .map_err(|_| "发送请求失败".to_string());
        
        //     // 处理接收响应
        //     match resp_rx.await {
        //         Ok(_) => {
        //         }
        //         Err(e) => {
        //             println!("日志发送失败 {}！",e);
        //         }
        //     }
        //   });
      }
      SensorsDataRequest::Cf3000(data)=>{
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
  StartMonitorProcess(oneshot::Sender<Result<(), String>>),
  StartProcess(oneshot::Sender<Result<(), String>>),
  StopProcess(oneshot::Sender<Result<(), String>>),
  SaveImageResult(Vec<u8>, String, oneshot::Sender<Result<(), String>>), // 保存图像结果
  SaveJsonResult(String, String, oneshot::Sender<Result<(), String>>),  // 保存 JSON 结果
  GetImageResult(Vec<u8>, String, oneshot::Sender<Result<String, String>>), // 获取图像结果,
  GetSensorResult(f64, oneshot::Sender<Result<String, String>>),  // 获取传感器结果
  SendLogToFrontend(String, oneshot::Sender<Result<(), String>>),  // 发送日志到前端
  SendImageToFrontend(Vec<u8>, oneshot::Sender<Result<(), String>>),  // 发送图像到前端
  GetCurrentState(oneshot::Sender<Result<String, String>>),  // 获取当前状态
  SendJsonToFrontend(String, oneshot::Sender<Result<(), String>>),
}

// 启动监控过程的异步任务
async fn monitor_process() -> Result<(), String> {
  let log = "[plc] [log] [开启PLC监控]";
  // sendlog2frontend(log.to_string());
  loop {
      // 每500ms读取某个PLC寄存器，模拟读取过程
      tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
      // 检测到停止按钮按下则退出
      // sendlog2frontend(log.to_string());
      println!("读取PLC寄存器...");
      // 检测到PLC启动信号则发送给主信号通道
  }
}

pub async fn start_global_task(mut rx: mpsc::Receiver<GeneralRequest>,app_handle: tauri::AppHandle) -> Result<(), String> {
  while let Some(request) = rx.recv().await {
    match request {
      GeneralRequest::StartMonitorProcess(resp_tx) => {
        // 启动异步任务
        tokio::spawn(async {
            let result = monitor_process().await;
            let _ = resp_tx.send(result);
        });

      }
      GeneralRequest::StartProcess(resp_tx)=>{

      }
      GeneralRequest::StopProcess(resp_tx)=>{

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
      // 获取图像结果
      GeneralRequest::GetImageResult(image_data, path, resp_tx) => {
        // 假设这里是返回处理结果
        let result = process_image(image_data, path).await;
        let _ = resp_tx.send(result);
      }
      // 获取传感器结果
      GeneralRequest::GetSensorResult(sensor_value, resp_tx) => {
        // 假设这里是传感器结果的处理逻辑
        let result = handle_sensor(sensor_value).await;
        let _ = resp_tx.send(result);
      }
      // 发送日志到前端
      GeneralRequest::SendLogToFrontend(log_message, resp_tx) => {
        // 假设这里是发送日志到前端的逻辑
        send_log_to_frontend(app_handle.clone(),log_message).await;
        let _ = resp_tx.send(Ok(()));
      }
      // 发送图像到前端
      GeneralRequest::SendImageToFrontend(image_data, resp_tx) => {
        // 假设这里是发送图像到前端的逻辑
        send_image_to_frontend(image_data).await;
        let _ = resp_tx.send(Ok(()));
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

async fn process_image(image_data: Vec<u8>, path: String) -> Result<String, String> {
  // 模拟图像处理逻辑
  println!("处理图像: {}，路径: {}", image_data.len(), path);
  Ok("图像处理成功".to_string())
}

async fn handle_sensor(sensor_value: f64) -> Result<String, String> {
  // 模拟传感器数据处理
  println!("处理传感器数据: {}", sensor_value);
  Ok("传感器数据处理成功".to_string())
}

async fn send_log_to_frontend(app_handle:tauri::AppHandle, log_message: String) {
  // 通过 Tauri 事件系统向前端发送日志信息
  app_handle.emit("log_received", log_message.clone()).unwrap();
  println!("发送日志到前端: {}", log_message);
}

async fn send_image_to_frontend(image_data: Vec<u8>) {
  // 模拟发送图像到前端
  println!("发送图像到前端，图像大小: {} bytes", image_data.len());
}

async fn send_json_to_frontend(result: String) {
  // 模拟发送图像到前端
  println!("发送图像到前端，图像大小: {} bytes", result);
}



// =========================================== 机器人相关 ==================================
// 机器人连接

// 写入机器人

// 读取机器人


// ========================================================================================


// =========================================== 算法相关 ====================================
// 算法相关代码位于sidecar模块内
// ========================================================================================

// =========================================== 其他 ========================================
// 前端类型选择
#[tauri::command]
fn select_artifact_type(start_state: &str) -> String {
        String::from(start_state) + "selected"
}

// 访问当前选择类型
#[tauri::command]
fn selected_artifact_type(start_state: &str) -> String {
        String::from(start_state) + "selected"
}

#[tauri::command]
fn serialize_node_config() -> String {
    println!("I was invoked from JavaScript!");
    "Call from frontend: test ok".to_string()
}

#[tauri::command]
fn serialize_project_config() -> String {
    println!("I was invoked from JavaScript!");
    "Call from frontend: test ok".to_string()
}
// =========================================================================================

// =========================================== 整体逻辑 ====================================
// 启动或终止后端程序
#[tauri::command(rename_all = "snake_case")]
fn start_software(start_state: String) -> String {
    match start_state.as_str(){
      // 启动后端流程
      "start" => {
        let mut state = START_STATE.lock().unwrap();
        *state = SoftwareState::START;
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
async fn start_get_data_mpsc(){}



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

// ========================================================================================


// =========================================== plc相关======================================
async fn start_plc_connect(plc_addr: std::net::SocketAddr) -> Result<bool, String> {
  // 使用 tauri 的 async_runtime::mpsc::channel 创建通道
  let (tx, rx) = mpsc::channel::<modbusTCP::ModbusRequest>(32);

  let tx = Arc::new(Mutex::new(Some(tx))); // 用 Mutex 包装 tx
  // 将 tx 存储在全局变量中
  *PLC_TX.lock().await = Some(tx.lock().await.clone().unwrap());

  // 启动异步任务
  tokio::spawn(modbusTCP::start_plc_task(plc_addr.clone(), rx));

  Ok(true) // 成功返回 true
}

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



// 读取寄存器的函数
async fn read_register(reg_address: u16) -> Result<u16, String> {
  let (resp_tx, resp_rx) = oneshot::channel();  // 创建响应通道
  // 获取全局的 tx
  let tx = PLC_TX.lock().await.clone().unwrap_or_else(|| {
    panic!("PLC_TX is not initialized. Ensure that start_plc_connect() has been called.");
  });
  tx.send(modbusTCP::ModbusRequest::ReadRegister(reg_address, resp_tx)).await.map_err(|_| "发送请求失败".to_string())?;

  match resp_rx.await {
      Ok(Ok(value)) => Ok(value),  // 返回读取的寄存器值
      Ok(Err(err)) => Err(err),    // Modbus 读取失败
      Err(err) => Err("响应通道关闭".to_string()),  // 响应通道关闭
  }
}

#[tauri::command(rename_all = "snake_case")]
async fn read_register_frontend(reg_address: u16) -> Result<u16, String> {
    read_register(reg_address).await
}

async fn write_register(reg_address: u16, value: u16)-> Result<(), String>{
  let (resp_tx, resp_rx) = oneshot::channel();
  let tx = PLC_TX.lock().await.clone().unwrap();
  // 注意此处没有对消息发送出错的处理
  tx.send(modbusTCP::ModbusRequest::WriteRegister(reg_address, value, resp_tx)).await.map_err(|_| "发送请求失败".to_string())?;
  match resp_rx.await {
      Ok(Ok(())) => Ok(()),
      Ok(Err(e)) => Err(e.to_string()),
      Err(_) => Err("Modbus 响应通道关闭".to_string()),
  }
}

#[tauri::command(rename_all = "snake_case")]
async fn write_register_frontend(reg_address: u16, value: u16) -> Result<String, String> {
    // 调用原本的 write_register 函数
    match write_register(reg_address, value).await {
      Ok(()) => Ok("success".to_string()),  // 成功返回 success
      Err(_) => Err("failed".to_string()),  // 失败返回 failed
  }
}

async fn read_coil(coil_address: u16){
  let (resp_tx, resp_rx) = oneshot::channel();
  let tx = PLC_TX.lock().await.clone().unwrap();
  tx.send(modbusTCP::ModbusRequest::ReadCoil(coil_address, resp_tx)).await.unwrap();
  match resp_rx.await {
      Ok(Ok(value)) => println!("Coil 5 状态: {}", value),
      Ok(Err(e)) => println!("读取 Coil 失败: {}", e),
      Err(_) => println!("Modbus 响应通道关闭"),
  }
}

async fn write_coil(coil_address: u16, value: bool){
  let (resp_tx, resp_rx) = oneshot::channel();
  let tx = PLC_TX.lock().await.clone().unwrap();
  tx.send(modbusTCP::ModbusRequest::WriteCoil(coil_address, value, resp_tx)).await.unwrap();
  match resp_rx.await {
      Ok(Ok(())) => println!("Coil 写入成功"),
      Ok(Err(e)) => println!("Coil 写入失败: {}", e),
      Err(_) => println!("Modbus 响应通道关闭"),
  }
}


async fn stop_plc_connection() {
  let (resp_tx, resp_rx):(oneshot::Sender<Result<(), String>>, oneshot::Receiver<Result<(), String>>) = oneshot::channel();
  // 尝试获取 PLC_TX 锁，并确保 tx 可用
  let tx_guard = PLC_TX.lock().await;

  // 确保 tx 存在，并且通道未关闭
  if let Some(tx) = tx_guard.clone() {
      // 发送停止命令
      match tx.send(modbusTCP::ModbusRequest::STOP(resp_tx)).await {
          Ok(()) => {
              println!("成功发送停止命令");
          }
          Err(e) => {
              println!("发送停止命令失败: {:?}", e);
          }
      }
  } else {
      println!("PLC 连接通道已关闭");
  }
  
  // 这里处理接收到的响应
  match resp_rx.await {
      Ok(Ok(())) => println!("成功停止 PLC 任务"),
      Ok(Err(e)) => println!("停止 PLC 任务失败: {}", e),
      Err(_) => println!("接收停止信号失败"),
  }
}

fn start_plc_connection(){
  tauri::async_runtime::spawn(async {
    // 这里可以执行一些后台任务
    println!("创建modbus tcp连接...");
    // plc启动
    let plc_addr: std::net::SocketAddr = "192.168.1.88:502".parse().unwrap();
    start_plc_connect(plc_addr).await;
    println!("modbus tcp连接创建完毕");
  });
}
// ==========================================================================================
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

fn start_global_mpsc_(app_handle: tauri::AppHandle){
  tauri::async_runtime::spawn(async move{
    // 这里可以执行一些后台任务
    println!("创建modbus tcp连接...");
    // plc启动
    start_global_mpsc(app_handle.clone()).await;
    println!("modbus tcp连接创建完毕");
  });
}

// ============================================== tauri相关 ==================================
fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
  app.manage(Arc::new(Mutex::new(None::<Arc<Mutex<Child>>>)));
  // Clone the app handle for use elsewhere
  let app_handle = app.handle().clone();

  println!("前端窗口已加载，启动后台任务");
  println!("[tauri] Creating fastapi sidecar...");
  sidecar::sidecar::spawn_and_monitor_sidecar(app_handle.clone()).ok();
  println!("[tauri] Fastapi Sidecar spawned and monitoring started.");
  // 启动modbus tcp异步通道
  start_plc_connection();
  start_global_mpsc_(app_handle.clone());
  start_sensor_mpsc().expect("Failed to start sensor mpsc");



  // 启动相机
  init_mvs_sdk();

  // 启动机器人异步通道
  tauri::async_runtime::spawn(async {
    // 启动PLC监控程序
    let (resp_tx, resp_rx) = oneshot::channel(); 
    let tx = GLOBAL_TX.lock().await.clone().unwrap_or_else(|| {
      panic!("GLOBAL_TX is not initialized. Ensure that start_global_task() has been called.");
    });
    tx.send(GeneralRequest::StartMonitorProcess(resp_tx)).await.map_err(|_| "发送请求失败".to_string());
  });

  tauri::async_runtime::spawn(async move{
    start_get_data_mpsc().await;
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
  });

  // match get_oneframe_timeout().await {
  //   Ok((buffer, frame_info)) => {
  //       let width = frame_info.nWidth as i32;
  //       let height = frame_info.nHeight as i32;
  //       println!("1");
  //       // 创建一个空 Mat
  //       let mut mat = unsafe {
  //         Mat::new_rows_cols(height, width, CV_8U).map_err(|_| "Mat 创建失败")?
  //       };
        
  //       // 获取 Mat 数据指针
  //       let mat_ptr = mat.data_mut();
  //       if mat_ptr.is_null() {
  //           return Err("Mat 数据指针为空");
  //       }

  //       // 复制 buffer 数据到 Mat
  //       unsafe {
  //           std::ptr::copy_nonoverlapping(buffer.as_ptr(), mat_ptr, buffer.len());
  //       }

  //       // 创建一个空 Mat 用于存放 RGB 图像数据
  //       let mut rgb_mat = Mat::new_rows_cols_with_default(
  //         height,
  //         width,
  //         CV_8UC3,
  //         Scalar::all(0.0),
  //       ).map_err(|_| "RGB Mat 创建失败")?;
        
  //       println!("2");
  //       // 将 Bayer 格式转换为 RGB
  //       imgproc::cvt_color(&mat, &mut rgb_mat, imgproc::COLOR_BayerGB2BGR, 0,AlgorithmHint::ALGO_HINT_DEFAULT)
  //       .map_err(|_| "Bayer 到 RGB 转换失败")?;
  //       println!("3");
  //       // OpenCV `imwrite` 保存图片
  //       let filename = "output.jpg";
  //       imgcodecs::imwrite(filename, &rgb_mat, &opencv::core::Vector::new()).map_err(|_| "保存图片失败")?;

  //       println!("图片保存成功: {}", filename);
  //       Ok(())
  //   }
  //   Err(e) => {
  //       eprintln!("获取帧失败: {}", e);
  //       Err("获取图像失败")
  //   }
  // }

  // 启动后端任务

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
              println!("modbus tcp 资源清理完成");
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
                                            read_register_frontend,
                                            write_register_frontend,
                                            sidecar::sidecar::start_sidecar,
                                            sidecar::sidecar::shutdown_sidecar])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
// ========================================================================================


