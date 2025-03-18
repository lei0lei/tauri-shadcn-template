use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager, RunEvent};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;
use std::process::{Command, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader}; // 引入 tokio 的异步读取特性
use tauri::{AppHandle};
use tokio::process::Command as TokioCommand; // 使用 tokio 版本的 Command
use tokio::task;


// Helper function to spawn the sidecar and monitor its stdout/stderr
pub fn spawn_and_monitor_sidecar(app_handle: tauri::AppHandle) -> Result<(), String> {
    // Check if a sidecar process already exists
    if let Some(state) = app_handle.try_state::<Arc<Mutex<Option<Arc<Mutex<tokio::process::Child>>>>>>() {
        let child_process = state.lock().unwrap();
        if child_process.is_some() {
            // A sidecar is already running, do not spawn a new one
            println!("[tauri] Sidecar is already running. Skipping spawn.");
            return Ok(()); // Exit early since sidecar is already running
        }
    }
    // Spawn sidecar
    let python_interpreter = if cfg!(target_os = "windows") {
        // 虚拟环境目录
        "D:\\github\\tauri-shadcn-template\\fastapi\\app\\.venv\\Scripts\\python.exe" // Windows
    } else {
        ".venv/bin/python" // Linux/macOS
    };

    // Path to your Python script
    // 启动fastapi
    let mut sidecar_command = TokioCommand::new(python_interpreter)
        .arg("-m")
        .arg("uvicorn")  // 只指定 uvicorn 模块
        .arg("app.app:app") // 将 FastAPI 应用传给 uvicorn
        .stdout(Stdio::piped()) // 捕获标准输出
        .stderr(Stdio::piped()) // 捕获错误输出
        .spawn()
        .map_err(|e| e.to_string())?;
 
    let sidecar_command = Arc::new(Mutex::new(sidecar_command));
    // Store the child process in the app state
    if let Some(state) = app_handle.try_state::<Arc<Mutex<Option<Arc<Mutex<tokio::process::Child>>>>>>() {
        println!("State acquired successfully");
        *state.lock().unwrap() = Some(sidecar_command.clone());
    } else {
        return Err("Failed to access app state".to_string());
    }
    // Clone the app_handle here to move it into async block
    let app_handle = app_handle.clone();
    
    // Spawn an async task to handle sidecar communication
    tauri::async_runtime::spawn({
        let app_handle = app_handle.clone(); // Clone the Arc here
        let sidecar_command = sidecar_command.clone(); // Clone the Arc here
        async move {
            // We must take the stdout and stderr to move them into tasks
            let mut sidecar_command = sidecar_command.lock().unwrap(); // Lock to access the child process
            let mut stdout = sidecar_command.stdout.take().unwrap();
            let mut stderr = sidecar_command.stderr.take().unwrap();

            let mut stdout_reader = BufReader::new(stdout);
            let mut stderr_reader = BufReader::new(stderr);

            // Monitor stdout of the sidecar process
            tokio::spawn({
                let app_handle = app_handle.clone(); // Clone the Arc here
                async move {
                    let mut line = String::new();
                    while stdout_reader.read_line(&mut line).await.unwrap() > 0 {
                        println!("Sidecar stdout: {}", line);
                        println!("Emitting sidecar-stdout event with payload: {}", line);
                        // Emit the line to the frontend (directly without locking)
                        app_handle
                            .emit("sidecar-stdout", line.clone())
                            .expect("Failed to emit sidecar stdout event");
                        line.clear();
                    }
                }
            });

            // Monitor stderr of the sidecar process
            tokio::spawn({
                let app_handle = app_handle.clone(); // Clone the Arc here
                async move {
                    let mut line = String::new();
                    while stderr_reader.read_line(&mut line).await.unwrap() > 0 {
                        eprintln!("Sidecar stdout: {}", line);

                        // Emit the line to the frontend (directly without locking)
                        app_handle
                            .emit("sidecar-stdout", line.clone())
                            .expect("Failed to emit sidecar stderr event");
                        line.clear();
                    }
                }
            });
        }
    });

    Ok(())
}

// Define a command to shutdown sidecar process
#[tauri::command]
pub fn shutdown_sidecar(app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("[tauri] Received command to shutdown sidecar.");
    // Access the sidecar process state
    if let Some(state) = app_handle.try_state::<Arc<Mutex<Option<CommandChild>>>>() {
        let mut child_process = state
            .lock()
            .map_err(|_| "[tauri] Failed to acquire lock on sidecar process.")?;

        if let Some(mut process) = child_process.take() {
            let command = "sidecar shutdown\n"; // Add newline to signal the end of the command

            // Attempt to write the command to the sidecar's stdin
            if let Err(err) = process.write(command.as_bytes()) {
                println!("[tauri] Failed to write to sidecar stdin: {}", err);
                // Restore the process reference if shutdown fails
                *child_process = Some(process);
                return Err(format!("Failed to write to sidecar stdin: {}", err));
            }

            println!("[tauri] Sent 'sidecar shutdown' command to sidecar.");
            Ok("'sidecar shutdown' command sent.".to_string())
        } else {
            println!("[tauri] No active sidecar process to shutdown.");
            Err("No active sidecar process to shutdown.".to_string())
        }
    } else {
        Err("Sidecar process state not found.".to_string())
    }
}



// Define a command to start sidecar process.
#[tauri::command]
pub fn start_sidecar(app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("[tauri] Received command to start sidecar.");
    spawn_and_monitor_sidecar(app_handle)?;
    Ok("Sidecar spawned and monitoring started.".to_string())
}


// 算法调用