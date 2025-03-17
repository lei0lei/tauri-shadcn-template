use std::net::SocketAddr;
use tokio::{sync::mpsc, time::Duration};
use tokio_modbus::client::tcp;
use tokio_modbus::prelude::*;
use tokio::sync::oneshot;


// 定义 PLC 读写请求类型
pub enum ModbusRequest {
    ReadRegister(u16, oneshot::Sender<Result<u16, String>>),    // 读取保持寄存器
    WriteRegister(u16, u16, oneshot::Sender<Result<(), String>>), // 写入保持寄存器
    ReadCoil(u16, oneshot::Sender<Result<bool, String>>),       // 读取 Coil
    WriteCoil(u16, bool, oneshot::Sender<Result<(), String>>),  // 写入 Coil
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
                        // println!("line28");
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
