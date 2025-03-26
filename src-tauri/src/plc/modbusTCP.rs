#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::net::SocketAddr;
use tokio::{sync::mpsc, time::Duration, sync::Mutex};
use tokio_modbus::client::tcp;
use tokio_modbus::prelude::*;
use tokio::sync::oneshot;
use std::sync::Arc;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref PLC_TX: Arc<Mutex<Option<mpsc::Sender<ModbusRequest>>>> = Arc::new(Mutex::new(None));
  }
  
lazy_static! {
pub static ref ROBOT_TX: Arc<Mutex<Option<mpsc::Sender<ModbusRequest>>>> = Arc::new(Mutex::new(None));
}

// 定义 PLC 读写请求类型
pub enum ModbusRequest {
    // 读取保持寄存器
    ReadRegister(u16, oneshot::Sender<Result<u16, String>>),
    // 写入保持寄存器
    WriteRegister(u16, u16, oneshot::Sender<Result<(), String>>),
    // 读取线圈
    ReadCoil(u16, oneshot::Sender<Result<bool, String>>),
    // 写入线圈
    WriteCoil(u16, bool, oneshot::Sender<Result<(), String>>),
    // 读取多个保持寄存器
    ReadMultipleRegisters(u16, u16, oneshot::Sender<Result<Vec<u16>, String>>),
    // 写入多个保持寄存器
    WriteMultipleRegisters(u16, Vec<u16>, oneshot::Sender<Result<(), String>>),
    // 读取多个线圈
    ReadMultipleCoils(u16, u16, oneshot::Sender<Result<Vec<bool>, String>>),
    // 写入多个线圈
    WriteMultipleCoils(u16, Vec<bool>, oneshot::Sender<Result<(), String>>),
    // 掩码写入寄存器
    MaskedWriteRegister(u16, u16, u16, oneshot::Sender<Result<(), String>>),
    // 读取离散输入
    ReadDiscreteInputs(u16, u16, oneshot::Sender<Result<Vec<bool>, String>>),
    // 读取输入寄存器
    ReadInputRegisters(u16, u16, oneshot::Sender<Result<Vec<u16>, String>>),
    // 读写多个寄存器
    ReadWriteMultipleRegisters(u16, u16, u16, Vec<u16>, oneshot::Sender<Result<Vec<u16>, String>>),

    // 中止modbusTCP响应任务
    STOP(oneshot::Sender<Result<(), String>>),
}


// 启动 PLC 任务
pub async fn start_plc_task(plc_addr: SocketAddr, mut rx: mpsc::Receiver<ModbusRequest>) -> Result<(), String> {
    
    match tcp::connect(plc_addr).await {
        Ok(mut ctx) => {
            println!("PLC 连接成功");
            while let Some(request) = rx.recv().await {
                match request {
                    // 读取保持寄存器
                    ModbusRequest::ReadRegister(reg, resp_tx) => {
                        let result = ctx.read_holding_registers(reg, 1).await
                            .map_err(|e| e.to_string())  // 处理 tokio_modbus::Error
                            .and_then(|inner_result| inner_result.map_err(|e| e.to_string()))  // 处理 ExceptionCode
                            .and_then(|vals| vals.get(0).copied().ok_or_else(|| "寄存器值为空".to_string()));  // 获取 Vec<u16> 的第一个元素
                        let _ = resp_tx.send(result);
                        }
                    // 写入保持寄存器
                    ModbusRequest::WriteRegister(reg, value, resp_tx) => {
                        let result = ctx.write_single_register(reg, value).await
                            .map_err(|e| e.to_string())  // 处理 tokio_modbus::Error
                            .and_then(|inner_result| inner_result.map_err(|e| e.to_string()).or(Ok(()))); // 处理 ExceptionCode，确保最终是 Result<(), String>
                        let _ = resp_tx.send(result);
                    }
                    // 读取 Coil
                    ModbusRequest::ReadCoil(addr, resp_tx) => {
                        let result = ctx.read_coils(addr, 1).await
                            .map_err(|e| e.to_string())  // 处理外层 tokio_modbus::Error
                            .and_then(|inner_result| inner_result.map_err(|e| e.to_string()))  // 处理 ExceptionCode
                            .and_then(|vals| vals.get(0).copied().ok_or_else(|| "Coil 读取为空".to_string()));  // 安全地获取第一个元素
                        let _ = resp_tx.send(result);
                    }
                    // 写入 Coil
                    ModbusRequest::WriteCoil(addr, value, resp_tx) => {
                        let result = ctx.write_single_coil(addr, value).await
                            .map_err(|e| e.to_string())  // 处理 tokio_modbus::Error
                            .and_then(|inner_result| inner_result.map_err(|e| e.to_string()).or(Ok(()))); // 处理 ExceptionCode
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::ReadMultipleRegisters(start, count, resp_tx) => {
                        let result: Result<Vec<u16>, String> = ctx.read_holding_registers(start, count).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::WriteMultipleRegisters(start, values, resp_tx) => {
                        let result: Result<(), String> = ctx.write_multiple_registers(start, &values).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()).map(|_| ()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::ReadMultipleCoils(start, count, resp_tx) => {
                        let result: Result<Vec<bool>, String> = ctx.read_coils(start, count).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::WriteMultipleCoils(start, values, resp_tx) => {
                        let result: Result<(), String> = ctx.write_multiple_coils(start, &values).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()).map(|_| ()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::MaskedWriteRegister(addr, and_mask, or_mask, resp_tx) => {
                        let result: Result<(), String> = ctx.masked_write_register(addr, and_mask, or_mask).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()).map(|_| ()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::ReadDiscreteInputs(start, count, resp_tx) => {
                        let result: Result<Vec<bool>, String> = ctx.read_discrete_inputs(start, count).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::ReadInputRegisters(start, count, resp_tx) => {
                        let result: Result<Vec<u16>, String> = ctx.read_input_registers(start, count).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::ReadWriteMultipleRegisters(read_start, read_count, write_start, values, resp_tx) => {
                        let result: Result<Vec<u16>, String> = ctx.read_write_multiple_registers(read_start, read_count, write_start, &values).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()));
                        let _ = resp_tx.send(result);
                    }
                    // 收到停止信号停止plc连接
                    ModbusRequest::STOP(resp_tx) => {
                        println!("接收到 STOP 信号，终止任务");
                        let _ = resp_tx.send(Ok(())); // 发送成功停止的信号
                        break; // 直接跳出内层循环，结束任务
                    }
                }
            }
            println!("所有 `Sender` 已关闭，退出 PLC 任务 关闭plc连接");
            Ok(()) // 任务完成，成功返回
        }
        Err(e) => {
            // 连接失败时直接返回错误
            Err(format!("PLC 连接失败: {:?}, 请检查网络连接或 PLC 配置", e))
        }
    }

}

// 启动 PLC 任务
pub async fn start_robot_task(plc_addr: SocketAddr, mut rx: mpsc::Receiver<ModbusRequest>) -> Result<(), String> {
    
    match tcp::connect(plc_addr).await {
        Ok(mut ctx) => {
            println!("PLC 连接成功");
            while let Some(request) = rx.recv().await {
                match request {
                    // 读取保持寄存器
                    ModbusRequest::ReadRegister(reg, resp_tx) => {
                        let result = ctx.read_holding_registers(reg, 1).await
                            .map_err(|e| e.to_string())  // 处理 tokio_modbus::Error
                            .and_then(|inner_result| inner_result.map_err(|e| e.to_string()))  // 处理 ExceptionCode
                            .and_then(|vals| vals.get(0).copied().ok_or_else(|| "寄存器值为空".to_string()));  // 获取 Vec<u16> 的第一个元素
                        let _ = resp_tx.send(result);
                        }
                    // 写入保持寄存器
                    ModbusRequest::WriteRegister(reg, value, resp_tx) => {
                        let result = ctx.write_single_register(reg, value).await
                            .map_err(|e| e.to_string())  // 处理 tokio_modbus::Error
                            .and_then(|inner_result| inner_result.map_err(|e| e.to_string()).or(Ok(()))); // 处理 ExceptionCode，确保最终是 Result<(), String>
                        let _ = resp_tx.send(result);
                    }
                    // 读取 Coil
                    ModbusRequest::ReadCoil(addr, resp_tx) => {
                        let result = ctx.read_coils(addr, 1).await
                            .map_err(|e| e.to_string())  // 处理外层 tokio_modbus::Error
                            .and_then(|inner_result| inner_result.map_err(|e| e.to_string()))  // 处理 ExceptionCode
                            .and_then(|vals| vals.get(0).copied().ok_or_else(|| "Coil 读取为空".to_string()));  // 安全地获取第一个元素
                        let _ = resp_tx.send(result);
                    }
                    // 写入 Coil
                    ModbusRequest::WriteCoil(addr, value, resp_tx) => {
                        let result = ctx.write_single_coil(addr, value).await
                            .map_err(|e| e.to_string())  // 处理 tokio_modbus::Error
                            .and_then(|inner_result| inner_result.map_err(|e| e.to_string()).or(Ok(()))); // 处理 ExceptionCode
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::ReadMultipleRegisters(start, count, resp_tx) => {
                        let result: Result<Vec<u16>, String> = ctx.read_holding_registers(start, count).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::WriteMultipleRegisters(start, values, resp_tx) => {
                        let result: Result<(), String> = ctx.write_multiple_registers(start, &values).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()).map(|_| ()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::ReadMultipleCoils(start, count, resp_tx) => {
                        let result: Result<Vec<bool>, String> = ctx.read_coils(start, count).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::WriteMultipleCoils(start, values, resp_tx) => {
                        let result: Result<(), String> = ctx.write_multiple_coils(start, &values).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()).map(|_| ()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::MaskedWriteRegister(addr, and_mask, or_mask, resp_tx) => {
                        let result: Result<(), String> = ctx.masked_write_register(addr, and_mask, or_mask).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()).map(|_| ()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::ReadDiscreteInputs(start, count, resp_tx) => {
                        let result: Result<Vec<bool>, String> = ctx.read_discrete_inputs(start, count).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::ReadInputRegisters(start, count, resp_tx) => {
                        let result: Result<Vec<u16>, String> = ctx.read_input_registers(start, count).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()));
                        let _ = resp_tx.send(result);
                    }
                    ModbusRequest::ReadWriteMultipleRegisters(read_start, read_count, write_start, values, resp_tx) => {
                        let result: Result<Vec<u16>, String> = ctx.read_write_multiple_registers(read_start, read_count, write_start, &values).await
                            .map_err(|e| e.to_string())
                            .and_then(|inner| inner.map_err(|e| e.to_string()));
                        let _ = resp_tx.send(result);
                    }
                    // 收到停止信号停止plc连接
                    ModbusRequest::STOP(resp_tx) => {
                        println!("接收到 STOP 信号，终止任务");
                        let _ = resp_tx.send(Ok(())); // 发送成功停止的信号
                        break; // 直接跳出内层循环，结束任务
                    }
                }
            }
            println!("所有 `Sender` 已关闭，退出 PLC 任务 关闭plc连接");
            Ok(()) // 任务完成，成功返回
        }
        Err(e) => {
            // 连接失败时直接返回错误
            Err(format!("PLC 连接失败: {:?}, 请检查网络连接或 PLC 配置", e))
        }
    }

}



// 读取寄存器的函数
pub async fn read_register_plc(reg_address: u16) -> Result<u16, String> {
    let (resp_tx, resp_rx) = oneshot::channel();  // 创建响应通道
    // 获取全局的 tx
    let tx = PLC_TX.lock().await.clone().unwrap_or_else(|| {
      panic!("PLC_TX is not initialized. Ensure that start_plc_connect() has been called.");
    });
    tx.send(ModbusRequest::ReadRegister(reg_address, resp_tx)).await.map_err(|_| "发送请求失败".to_string())?;
  
    match resp_rx.await {
        Ok(Ok(value)) => Ok(value),  // 返回读取的寄存器值
        Ok(Err(err)) => Err(err),    // Modbus 读取失败
        Err(err) => Err("响应通道关闭".to_string()),  // 响应通道关闭
    }
  }

// 读取寄存器的函数
pub async fn read_register_robot(reg_address: u16) -> Result<u16, String> {
    let (resp_tx, resp_rx) = oneshot::channel();  // 创建响应通道
    // 获取全局的 tx
    let tx = ROBOT_TX.lock().await.clone().unwrap_or_else(|| {
      panic!("PLC_TX is not initialized. Ensure that start_plc_connect() has been called.");
    });
    tx.send(ModbusRequest::ReadRegister(reg_address, resp_tx)).await.map_err(|_| "发送请求失败".to_string())?;
  
    match resp_rx.await {
        Ok(Ok(value)) => Ok(value),  // 返回读取的寄存器值
        Ok(Err(err)) => Err(err),    // Modbus 读取失败
        Err(err) => Err("响应通道关闭".to_string()),  // 响应通道关闭
    }
  }


pub async fn write_register_plc(reg_address: u16, value: u16)-> Result<(), String>{
    let (resp_tx, resp_rx) = oneshot::channel();
    let tx = PLC_TX.lock().await.clone().unwrap();
    // 注意此处没有对消息发送出错的处理
    tx.send(ModbusRequest::WriteRegister(reg_address, value, resp_tx)).await.map_err(|_| "发送请求失败".to_string())?;
    match resp_rx.await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e.to_string()),
        Err(_) => Err("Modbus 响应通道关闭".to_string()),
    }
  }


pub async fn write_register_robot(reg_address: u16, value: u16)-> Result<(), String>{
    let (resp_tx, resp_rx) = oneshot::channel();
    let tx = ROBOT_TX.lock().await.clone().unwrap();
    // 注意此处没有对消息发送出错的处理
    tx.send(ModbusRequest::WriteRegister(reg_address, value, resp_tx)).await.map_err(|_| "发送请求失败".to_string())?;
    match resp_rx.await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e.to_string()),
        Err(_) => Err("Modbus 响应通道关闭".to_string()),
    }
  }



pub async fn read_coil(coil_address: u16){
    let (resp_tx, resp_rx) = oneshot::channel();
    let tx = PLC_TX.lock().await.clone().unwrap();
    tx.send(ModbusRequest::ReadCoil(coil_address, resp_tx)).await.unwrap();
    match resp_rx.await {
        Ok(Ok(value)) => println!("Coil 5 状态: {}", value),
        Ok(Err(e)) => println!("读取 Coil 失败: {}", e),
        Err(_) => println!("Modbus 响应通道关闭"),
    }
  }
  
pub async fn write_coil(coil_address: u16, value: bool){
    let (resp_tx, resp_rx) = oneshot::channel();
    let tx = PLC_TX.lock().await.clone().unwrap();
    tx.send(ModbusRequest::WriteCoil(coil_address, value, resp_tx)).await.unwrap();
    match resp_rx.await {
        Ok(Ok(())) => println!("Coil 写入成功"),
        Ok(Err(e)) => println!("Coil 写入失败: {}", e),
        Err(_) => println!("Modbus 响应通道关闭"),
    }
  }


pub async fn start_plc_connect(plc_addr: std::net::SocketAddr) -> Result<bool, String> {
    // 使用 tauri 的 async_runtime::mpsc::channel 创建通道
    let (tx, rx) = mpsc::channel::<ModbusRequest>(32);
  
    let tx = Arc::new(Mutex::new(Some(tx))); // 用 Mutex 包装 tx
    // 将 tx 存储在全局变量中
    *PLC_TX.lock().await = Some(tx.lock().await.clone().unwrap());
  
    // 启动异步任务
    tokio::spawn(start_plc_task(plc_addr.clone(), rx));
  
    Ok(true) // 成功返回 true
  }
  
pub async fn start_robot_connect(plc_addr: std::net::SocketAddr) -> Result<bool, String> {
    // 使用 tauri 的 async_runtime::mpsc::channel 创建通道
    let (tx, rx) = mpsc::channel::<ModbusRequest>(32);
  
    let tx = Arc::new(Mutex::new(Some(tx))); // 用 Mutex 包装 tx
    // 将 tx 存储在全局变量中
    *ROBOT_TX.lock().await = Some(tx.lock().await.clone().unwrap());
  
    // 启动异步任务
    tokio::spawn(start_robot_task(plc_addr.clone(), rx));
  
    Ok(true) // 成功返回 true
  }

fn get_plc_ip_port() -> Option<String> {
    // 获取全局配置
    let config = crate::config::config::CONFIG.read().unwrap();  // 获取只读锁
  
    // 检查配置是否已经加载
    if let Some(config) = &*config {
        // 访问硬件配置中的 plc 子配置项
        if let Some(plc_ip_port) = config.hardware.get_value("plc.ip_port") {
            if let Some(plc_addr_str) = plc_ip_port.as_str(){
                // 返回ip_port
                return Some(plc_addr_str.to_string());
              } else {
                println!("PLC ip_port 不是字符串类型");
              }
        } else {
            println!("PLC ip_port not found.");
        }
    } else {
        println!("配置加载失败");
    }
    None  // 如果找不到，返回 None
  }

fn get_robot_ip_port() -> Option<String> {
    // 获取全局配置
    let config = crate::config::config::CONFIG.read().unwrap();  // 获取只读锁
  
    // 检查配置是否已经加载
    if let Some(config) = &*config {
        // 访问硬件配置中的 plc 子配置项
        if let Some(robot_ip_port) = config.hardware.get_value("robot.ip_port") {
            if let Some(robot_addr_str) = robot_ip_port.as_str(){
                // 返回ip_port
                return Some(robot_addr_str.to_string());
              } else {
                println!("PLC ip_port 不是字符串类型");
              }
        } else {
            println!("PLC ip_port not found.");
        }
    } else {
        println!("配置加载失败");
    }
    None  // 如果找不到，返回 None
  }

pub fn start_plc_connection(){
    tauri::async_runtime::spawn(async {
      // 这里可以执行一些后台任务
      println!("创建modbus tcp连接...");
  
      if let Some(plc_addr) = get_plc_ip_port() {
        // 将读取到的 ip_port 转换为 SocketAddr 类型
        
        if let Ok(socket_addr) = plc_addr.parse::<SocketAddr>() {
            // 启动 PLC 连接
            start_plc_connect(socket_addr).await;
            println!("modbus tcp连接创建完毕");
        } else {
            println!("PLC ip_port 格式错误: {}", plc_addr);
        }
    } else {
        println!("PLC ip_port not found.");
    }
    });
  }
  
  
pub fn start_robot_connection(){
    tauri::async_runtime::spawn(async {
      // 这里可以执行一些后台任务
      println!("创建modbus tcp连接...");
  
      if let Some(robot_addr) = get_robot_ip_port() {
        // 将读取到的 ip_port 转换为 SocketAddr 类型
        
        if let Ok(socket_addr) = robot_addr.parse::<SocketAddr>() {
            // 启动 PLC 连接
            start_robot_connect(socket_addr).await;
            println!("modbus tcp连接创建完毕");
        } else {
            println!("PLC ip_port 格式错误: {}", robot_addr);
        }
    } else {
        println!("PLC ip_port not found.");
    }
    });
  }



pub async fn stop_plc_connection() {
    let (resp_tx, resp_rx):(oneshot::Sender<Result<(), String>>, oneshot::Receiver<Result<(), String>>) = oneshot::channel();
    // 尝试获取 PLC_TX 锁，并确保 tx 可用
    let tx_guard = PLC_TX.lock().await;
  
    // 确保 tx 存在，并且通道未关闭
    if let Some(tx) = tx_guard.clone() {
        // 发送停止命令
        match tx.send(ModbusRequest::STOP(resp_tx)).await {
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
  
  

pub fn set_plc_test() -> Result<(), Box<dyn std::error::Error>> {
    // 获取全局配置
    let mut config = match crate::config::config::CONFIG.write() {
        Ok(config) => config,
        Err(_) => {
            println!("获取配置失败");
            return Err("获取配置失败".into());
        }
    };
    if let Some(ref mut config) = *config {
      // 设置 plc.test 为 true
      let new_value = toml::Value::Boolean(true); // 设置为布尔值 true
      config.hardware.set_value("sensors.ip", new_value)?;
      Ok(())
  } else {
      Err("配置为空".into())
  }
  }
  
  
pub fn remove_plc_test()-> Result<(), Box<dyn std::error::Error>> {
    let mut config = match crate::config::config::CONFIG.write() {
      Ok(config) => config,
      Err(_) => {
          println!("获取配置失败");
          return Err("获取配置失败".into());
      }
    };
    if let Some(ref mut config) = *config {
      // 设置 plc.test 为 true
      let new_value = toml::Value::Boolean(true); // 设置为布尔值 true
      config.hardware.remove_value("plc.test")?;
      Ok(())
    } else {
      Err("配置为空".into())
    }
  }
  
  
pub fn set_robot_test() -> Result<(), Box<dyn std::error::Error>> {
    // 获取全局配置
    let mut config = match crate::config::config::CONFIG.write() {
        Ok(config) => config,
        Err(_) => {
            println!("获取配置失败");
            return Err("获取配置失败".into());
        }
    };
    if let Some(ref mut config) = *config {
      // 设置 plc.test 为 true
      let new_value = toml::Value::Boolean(true); // 设置为布尔值 true
      config.hardware.set_value("robot.ip", new_value)?;
      Ok(())
    } else {
        Err("配置为空".into())
    }
  }
  
pub fn remove_robot_test()-> Result<(), Box<dyn std::error::Error>> {
    let mut config = match crate::config::config::CONFIG.write() {
      Ok(config) => config,
      Err(_) => {
          println!("获取配置失败");
          return Err("获取配置失败".into());
      }
    };
    if let Some(ref mut config) = *config {
      // 设置 plc.test 为 true
      let new_value = toml::Value::Boolean(true); // 设置为布尔值 true
      config.hardware.remove_value("robot.test")?;
      Ok(())
    } else {
      Err("配置为空".into())
    }
  }