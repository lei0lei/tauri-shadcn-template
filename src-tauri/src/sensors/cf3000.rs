extern crate libc;

use libc::c_int; 
use std::ptr;
use crate::sensors::cf3000_bindings;

use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use chrono::Local;
use super::super::SensorsDataRequest;
use super::super::GLOBAL_SENSOR_TX;
#[link(name = "cf_windows_3156557", kind = "dylib")] 
#[link(name = "hps_cfxxxx_sdk", kind = "dylib")] 

impl Default for cf3000_bindings::SC_ResultDataTypeDef_t {
    fn default() -> Self {
        cf3000_bindings::SC_ResultDataTypeDef_t {
            channelIndex: 0,
            saturation: 0.0,
            resultLen: 0,
            result: [0.0; 10],
            distanceNumber: 0,
            distance: [0.0; 20],
            signal: 0,
            signalLength: 0,
            triggerCount: 0,
            triggerCount1: 0,
            triggerCount2: 0,
            bTriggerPass: 0,
        }
    }
}


static mut g_measure_result: [f32; 4] = [0.0; 4];  // 测量结果
static mut single_channel_result: [cf3000_bindings::SC_ResultDataTypeDef_t; 4] = [cf3000_bindings::SC_ResultDataTypeDef_t {
    channelIndex: 0,
    saturation: 0.0,
    resultLen: 0,
    result: [0.0; 10],
    distanceNumber: 0,
    distance: [0.0; 20],
    signal: 0,
    signalLength: 0,
    triggerCount: 0,
    triggerCount1: 0,
    triggerCount2: 0,
    bTriggerPass: 0,
}; 4]; // 假设单通道数据结构数组长度为 4

static IS_RECEIVE_DATA: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

fn rs_CF_ScanDeviceList() {
    println!("scan device");
    unsafe {
        let mut dev_list: *mut *mut cf3000_bindings::DeviceInfo_t = ptr::null_mut();
        let mut device_number: i32 = 0;

        let status = cf3000_bindings::CF_ScanDeviceList(dev_list, &mut device_number);
        if device_number == 0 {
            println!("没有发现设备");
            return;
        }
    }
}

pub async fn rs_CF_GE_OpenDevice(ip: &str, device_handler: &mut cf3000_bindings::DeviceHandle_t,){
    println!("open device");

    unsafe {
        let controller_ip_ptr: *mut i8 = ptr::null_mut();
        let controller_mac_ptr: *mut i8 = ptr::null_mut();
        let controller_port: u16 = 0;

        let mut controller_para = cf3000_bindings::ControllerGEPara_t {
            controllerIp: controller_ip_ptr,
            controllerMAC: controller_mac_ptr,
            controllerPort: controller_port,
        };
        
        // let mut device_handler: bindings::DeviceHandle_t = -1;

        // 使用 into_raw 来获得一个 *mut i8
        let local_ip = std::ffi::CString::new(ip).expect("Failed to create ip CString");
        let local_ip_ptr = local_ip.into_raw();  // 转为 *mut i8

        let status = cf3000_bindings::CF_GE_OpenDevice(
            &mut controller_para,
            local_ip_ptr,
            device_handler,
            1,
        );

        println!("status: {}", status);

        // 记得释放 CString 转换后的指针
        // 防止内存泄漏
        let _ = std::ffi::CString::from_raw(local_ip_ptr); // 释放内存
    }
}

fn rs_CF_CloseDevice(device_handler: cf3000_bindings::DeviceHandle_t){
    println!("close device {device_handler}");
    // let mut device_handler: bindings::DeviceHandle_t = 0;
    unsafe{
        cf3000_bindings::CF_CloseDevice(device_handler);
    }
    println!("device closed")
}


pub async fn rs_CF_StartSample(device_handler: cf3000_bindings::DeviceHandle_t, cmd: bool){
    // let handle: DeviceHandle_t = 0;
    println!("device start sampling {device_handler}");
    unsafe{
        let status = cf3000_bindings::CF_StartSample(device_handler,cmd);
        // println!("{status}");
        if status == 0 {
            println!("Sensor sampling started successfully.");
        }else{
            println!("Sensor sampling started failed.");
        }
    }
    
}

fn rs_CF_DarkSignal(){


}


fn rs_CF_Zero(){


}

unsafe extern "C" fn rs_event_callback(handle: cf3000_bindings::DeviceHandle_t, arg: cf3000_bindings::EventCallbackArgs_t, user_para: *mut std::ffi::c_void,){
    if arg.eventType == cf3000_bindings::EventTypeDef_EventType_DataRecv {
        if arg.rid == cf3000_bindings::ConfocalDataRid_t_RID_RESULT {
            // 获取接收到的结果数据
            if false {
                // 获取双通道数据的函数 (假设你已有实现)
                // get_double_channel_result(arg.data, arg.dataLen);
            } else {
                // 获取单通道数据
                rs_getSingleChannelResult(arg.data, arg.dataLen);
            }
            unsafe {
                let mut data_received = IS_RECEIVE_DATA.lock().unwrap();
                *data_received = true;
            }
        } else if arg.rid == cf3000_bindings::ConfocalDataRid_t_RID_DEVICE_DISCONNECT {
            println!("Device disconnected!");
        } else if arg.rid == cf3000_bindings::ConfocalDataRid_t_RID_API_CALL_EXCEPTION {
            // 打印错误信息
            let error_message = std::ffi::CStr::from_ptr(arg.data as *const i8);
            println!("API call exception: {:?}", error_message);
        }
    }
}


fn rs_getSingleChannelResult(data: *mut std::ffi::c_void, length: i32) {
    unsafe {
        if data.is_null() {
            return;
        }
        
        // 拷贝数据到 single_channel_result
        let size_of_single_result = std::mem::size_of::<cf3000_bindings::SC_ResultDataTypeDef_t>();
        let total_bytes = (length as usize) * size_of_single_result;
        std::ptr::copy_nonoverlapping(
            data as *const u8, // 将 data 视为字节数组
            single_channel_result.as_mut_ptr() as *mut u8, // 将目标视为字节数组
            total_bytes,
        );
        // std::ptr::copy_nonoverlapping(data as *const bindings::SC_ResultDataTypeDef_t, single_channel_result.as_mut_ptr(), length as usize);
        for i in 0..length {
            let index = single_channel_result[i as usize].channelIndex;
            g_measure_result[index as usize] = single_channel_result[i as usize].result[0]; // 将数据存到对应的索引
            // println!("{:?}", g_measure_result);
        }
        let current_time = Local::now();
        println!("当前时间: {}", current_time);
        println!("{:?}", g_measure_result);
        if let Some(tx) = GLOBAL_SENSOR_TX.lock().unwrap().as_ref() {
            let _ = tx.send(SensorsDataRequest::Cf3000(g_measure_result[0] as f64));
        }
    }
}

pub async fn rs_CF_RegisterEventCallback(){
    println!("register event callback");
    let rust_event_callback: unsafe extern "C" fn(cf3000_bindings::DeviceHandle_t, cf3000_bindings::EventCallbackArgs_t, *mut ::std::os::raw::c_void) = rs_event_callback;
    unsafe {
        cf3000_bindings::CF_RegisterEventCallback(Some(rust_event_callback), std::ptr::null_mut());
    }

}

fn rs_CF_GetLatestResult(handle: i32, res: &mut [cf3000_bindings::SC_ResultDataTypeDef_t; 4], len: &mut i32){

    unsafe {
        let ret = cf3000_bindings::CF_GetLatestResult(handle, res.as_mut_ptr(), len);
        // println!("{ret}")
    }
}



fn rs_CF_SetIntParam(handle: cf3000_bindings::DeviceHandle_t,
                    param_name: &[u8],
                    channel_index: c_int,
                    value: c_int,) -> i32 {
        unsafe {
            let ret = cf3000_bindings::CF_SetIntParam(handle, param_name.as_ptr() as *const i8, channel_index, value);
            ret
        }

}

fn rs_CF_GetIntParam(handle: cf3000_bindings::DeviceHandle_t,
                    param_name: &[u8],
                    channel_index: c_int,) -> Result<i32, cf3000_bindings::StatusTypeDef> {
    let mut value: c_int = 0;

    let ret = unsafe {
        cf3000_bindings::CF_GetIntParam(
            handle,
            param_name.as_ptr() as *const i8,
            channel_index,
            &mut value,
        )
    };

    if ret == cf3000_bindings::StatusTypeDef_Status_Succeed {
        Ok(value) // 返回获取的值
    } else {
        Err(ret) // 返回错误码
    }

}

// pub fn main_() {

    // std::env::set_var("LANG", "en_ZH.UTF-8");
    // 1-设备扫描

    // 7-回调函数注册
    

    // 2-网口设备打开
    // let ip = "192.168.0.101";
    // let mut device_handler: cf3000_bindings::DeviceHandle_t = -1;
    // rs_CF_GE_OpenDevice(ip,&mut device_handler);
    // println!("opened device: {device_handler}");

    // 4-启动/停止数据采集
    // let cmd:bool = true;
    // rs_CF_StartSample(device_handler,cmd);

    // for i in 0..30 {
    //     unsafe {
    //         println!("{:?}", g_measure_result);
    //         thread::sleep(Duration::from_secs_f64(0.1));
    //         let data_received = IS_RECEIVE_DATA.lock().unwrap();
    //         if *data_received {
    //                 println!("{:?}", g_measure_result);
    //         } else {
    //             println!("未开启数据采集")
    //         }
    //     }
    // }

    // // 8-获取最新一帧单头测量值
    // let mut signalLength: i32 = 0;

    // let mut single_result: [cf3000_bindings::SC_ResultDataTypeDef_t; 4] = Default::default();
    // for i in 0..10 {
    //     rs_CF_GetLatestResult(device_handler,&mut single_result,&mut signalLength);
    //     println!("{:?}",single_result[0].result[0]);
    // }

    // // 获取int型参数
    // let param_name = cf3000_bindings::PARAM_DOUBLE_CHANNEL_MODE;
    // let channel_index: c_int = 0;
    // let channelModeState: c_int = 0;

    // let ret = rs_CF_SetIntParam(device_handler, param_name,channel_index,channelModeState);
    // if ret != cf3000_bindings::StatusTypeDef_Status_Succeed {
    //     println!("设置单头模式失败，打印错误码：{:?}", ret);
    // } else {
    //     println!("单头模式设置成功！");
    // }

    // // 设置int型参数
    // // rs_CF_GetIntParam();
    // match rs_CF_GetIntParam(device_handler, param_name, channel_index) {
    //     Ok(value) => println!("获取到的值: {}", value),
    //     Err(error_code) => println!("获取参数失败，错误码：{}", error_code),
    // }




    // // 3-设备关闭
    // rs_CF_CloseDevice(device_handler);

    // // 5-执行dark操作
    // rs_CF_DarkSignal();

    // // 6-测量值归0
    // rs_CF_Zero();


    // 导出缓存中所有数据

    // 保存当前用户配置


    // 恢复出厂设置

    // 导出设备配置文件


    // 导入设备配置文件


    // 设置外部触发IO

    // 解绑指定输入IO


    // 获取float型参数


    // 设置float型参数

    // 获取string型参数


    // 设置string型参数



//     println!("Press Enter to exit...");
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
// }