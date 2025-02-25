// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::sync::{mpsc, Mutex};
use tauri_shadcn_template_lib::run;

// 主流程启动状态
static mut START_STATE: bool = false;



#[tokio::main]
async fn main() {
    // 运行 Tauri 后端
    run();
    // 每个循环表示一个工件的处理
    loop {
        // 生成处理函数

        break;
    }
}