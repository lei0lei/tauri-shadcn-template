// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]
use tokio::sync::{mpsc, Mutex};
use tauri_shadcn_template_lib::run_tauri_app;

mod sidecar;
use env_logger;


fn main() {
    // 运行 Tauri 后端
    env_logger::init();
    run_tauri_app();
    println!("如果你看到这条消息，说明tauri启动错误");
}
