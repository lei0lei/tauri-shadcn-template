// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// use tauri::{command, event, Manager};


use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;
use std::io::Read;
use tokio::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;
use tauri::{Manager};
use tokio::process::Child; // 导入 tokio::process::Child



mod sidecar;
// use tauri_shadcn_template_lib::sidecar::{shutdown_sidecar, start_sidecar};

// 主流程启动状态
pub static mut START_STATE: bool = false;


// 启动或终止后端程序
#[tauri::command]
fn start_software(start_state: &str) -> String {
    match start_state{
        // 启动后端流程
        "start" => {
          unsafe {
            START_STATE = true;
          }
          "started".to_string()
        },
        // 终止后端流程
        "end"   => {
          unsafe {
            START_STATE = false;
          }
          "ended".to_string()
        },
         _      => {
          "wrong param".to_string()
        },
    }
}

// 保存算法流程



// 保存项目流程



// 后端fastapi算法封装



// 前端简单调用测试
#[tauri::command]
fn test_call_from_frontend() -> String {
    println!("I was invoked from JavaScript!");
    "Call from frontend: test ok".to_string()
}

// 事件系统调用测试




// 前端图片显示调用测试
#[derive(Clone)]
struct AppState {
    sender: Sender<Vec<u8>>,
}


#[tauri::command]
fn test_image_transfer_to_frontend() -> String {
    println!("I was invoked from JavaScript!");
    "Call from frontend: test ok".to_string()
}



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


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run_tauri_app() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
      // Store the initial sidecar process in the app state
      // <Arc<Mutex<Option<Arc<Mutex<tokio::process::Child>>>>>>
      app.manage(Arc::new(Mutex::new(None::<Arc<Mutex<Child>>>)));
      // Clone the app handle for use elsewhere
      let app_handle = app.handle().clone();
      // Spawn the Python sidecar on startup
      println!("[tauri] Creating sidecar...");
      sidecar::sidecar::spawn_and_monitor_sidecar(app_handle).ok();
      println!("[tauri] Sidecar spawned and monitoring started.");
      Ok(())
  })
    .invoke_handler(tauri::generate_handler![start_software,
                                            test_call_from_frontend,
                                            sidecar::sidecar::start_sidecar,
                                            sidecar::sidecar::shutdown_sidecar])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


// 计算机硬件占用检测

struct Computer_state{


}


// 后端硬件状态检测
struct Hardware_State{
    

}

struct Hardware{


}


// 硬件配置、启动、停止、连接、异步数据获取、单次数据获取



// 启动数据库、相机、机器人、plc、传感器、算法后端服务



// 相关设置加载保存





// 