// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 先运行 Tauri 的相关设置
    tauri::Builder::default()
        // .plugin(tauri_plugin_os::init())     // 初始化操作系统插件
        // .plugin(tauri_plugin_window::init()) // 初始化窗口插件
        .run(tauri::generate_context!())     // 运行 Tauri 应用
        .expect("error while running tauri application");

    // 然后运行自定义的模板库
    tauri_shadcn_template_lib::run();
}