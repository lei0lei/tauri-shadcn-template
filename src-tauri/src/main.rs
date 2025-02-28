// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::sync::{mpsc, Mutex};
use tauri_shadcn_template_lib::run_tauri_app;

use tauri_shadcn_template_lib::START_STATE;
mod sidecar;
use env_logger;


fn main() {
    // 运行 Tauri 后端
    env_logger::init();
    run_tauri_app();
    // 硬件设备启动

    
    // fastapi后端启动


    // 开始检测流程，每个循环为一个工件的检测流程
    loop {
        // 生成处理函数

        break;
    }
}