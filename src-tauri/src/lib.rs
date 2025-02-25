// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn start_software(start_state: &str) -> String {
    match start_state{
        // 启动后端流程
        "start" => "started".to_string(),
        // 终止后端流程
        "end"   => "ended".to_string(),
         _      => "wrong param".to_string(),
    }
}

#[tauri::command]
fn test_call_from_frontend() -> String {
        "Call from frontend: test ok".to_string()
}

#[tauri::command]
fn selected_artifact_type(start_state: &str) -> String {
        String::from(start_state) + "selected"
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_software,
                                            test_call_from_frontend])
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
