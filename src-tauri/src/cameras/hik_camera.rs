#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use hikvision::Lib;
use hikvision::mvs_sdk::HcMvsCoreSdk;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use e_utils::parse::ParseResult;
use std::error::Error;
use lazy_static::lazy_static;
use e_utils::{auto_any_res_c, CResult};
use hikvision::mvs_sdk::types::*;
use std::rc::Rc;
use e_utils::parse::ToStringWithoutNulls as _;
use hikvision::mvs_sdk::{
    constant::INFO_MAX_BUFFER_SIZE,
    types::{MvCcDeviceInfoList, N19MvCCDeviceInfo3Dot0E,MvAccessMode,MvCcDeviceInfo,CbOutputCallback,MvFrameOutInfoEx,MvChunkDataContent},
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    ffi::*,
    net::{IpAddr, Ipv4Addr},
};

use super::super::SensorsDataRequest;
use super::super::GLOBAL_SENSOR_TX;
use super::super::GLOBAL_TX;
use tokio::task;


lazy_static! {
    // Global MVS Camera sdk
    pub static ref HC_MVS_CORE_SDK: Arc<RwLock<HcMvsCoreSdk>> =
      Arc::new(RwLock::new(HcMvsCoreSdk::default()));
  }

pub fn init_mvs_sdk() -> Result<(),Box<dyn Error>> {
    let lib = Lib::new(PathBuf::from("C:\\Program Files (x86)\\Common Files\\MVS\\Runtime\\Win64_x64\\MvCameraControl.dll"));
    HC_MVS_CORE_SDK.write().res()?.set_lib(lib);
    println!("Init hikvision SDK OK...");
    
    Ok(())
  }

pub async fn enumerate_devices(layerType: MvEnumDeviceLayerType) {
    let mut out_dev_list = MvCcDeviceInfoList::default();
    let msg = format!("工业摄像头 枚举设备");

    let mut sdk_guard = HC_MVS_CORE_SDK.write();
    
    // 处理 Result 类型，解包
    match sdk_guard {
        Ok(mut sdk) => {
            // 成功时，可以访问 sdk
            let res = unsafe {
                sdk.enumrate_devices(layerType, &mut out_dev_list)
            };
            let out_dev_list2 = MvCcDeviceInfoList2::from(&mut out_dev_list);
            println!("{:?}", out_dev_list2);
            // 使用写锁修改数据
            sdk.set_devlist(out_dev_list);
        }
        Err(e) => {
            // 处理写锁失败的情况
            eprintln!("获取 SDK 写锁失败: {:?}", e);
        }
    }
}

pub async fn is_device_accessable()-> Result<Vec<bool>, &'static str>{
    let mut sdk_guard = HC_MVS_CORE_SDK.write();

    match sdk_guard {
        Ok(mut sdk) => {
            // 成功时，可以访问 sdk
            let device_list = sdk.get_devlist();
            // 获取每个设备信息并检查访问权限
            let mut results = Vec::new();
            for device_info in &device_list.pDeviceInfo {
                
                // 调用 is_device_accessible 函数
                unsafe {
                    let accessible = sdk.is_device_accessible(&**device_info, MvAccessMode::Exclusive);
                    match accessible {
                        CResult::Ok(val) => results.push(val),    // 如果是 Ok，获取值并添加到结果中
                        CResult::Err(_) => results.push(false),   // 如果是 Err，使用默认值 false
                    }
                }
            }
            Ok(results)
        }
        Err(e) => {
            eprintln!("获取 SDK 写锁失败: {:?}", e);
            Err("获取 SDK 写锁失败") // 这里返回 Result 类型
        }
    }
}

pub async fn is_device_connected() -> Result<bool, &'static str> {
    let sdk_guard = HC_MVS_CORE_SDK.read(); // 获取读锁
    match sdk_guard {
        Ok(sdk) => {
            unsafe {
                match sdk.is_device_connected() {
                    CResult::Ok(connected) => Ok(connected), // 成功返回设备连接状态
                    CResult::Err(_) => Err("设备连接状态查询失败"),
                }
            }
        }
        Err(e) => {
            eprintln!("获取 SDK 读锁失败: {:?}", e);
            Err("获取 SDK 读锁失败")
        }
    }
}

pub async fn open_device(access_mode: MvAccessMode, switchover_key: c_ushort) -> Result<c_int, &'static str> {
    let mut sdk_guard = HC_MVS_CORE_SDK.write(); // 获取写锁
    match sdk_guard {
        Ok(mut sdk) => {
            // 调用底层 C 接口
            let result = unsafe { sdk.open_device(access_mode, switchover_key) };
            match result {
                CResult::Ok(code) => Ok(code),
                CResult::Err(_) => Err("打开设备失败"),
            }
        }
        Err(e) => {
            eprintln!("获取 SDK 写锁失败: {:?}", e);
            Err("获取 SDK 写锁失败")
        }
    }
}

pub async fn close_device() -> Result<c_int, &'static str> {
    let mut sdk_guard = HC_MVS_CORE_SDK.write(); // 获取写锁
    match sdk_guard {
        Ok(mut sdk) => {
            // 调用底层 C 接口
            let result = unsafe { sdk.close_device() };
            match result {
                CResult::Ok(code) => Ok(code),
                CResult::Err(_) => Err("关闭设备失败"),
            }
        }
        Err(e) => {
            eprintln!("获取 SDK 写锁失败: {:?}", e);
            Err("获取 SDK 写锁失败")
        }
    }
}

pub async fn start_grabbing() -> Result<c_int, &'static str> {
    let mut sdk_guard = HC_MVS_CORE_SDK.write(); // 获取写锁
    match sdk_guard {
        Ok(mut sdk) => {
            // 调用底层 C 接口
            let result = unsafe { sdk.start_grabbing() };
            match result {
                CResult::Ok(code) => Ok(code),
                CResult::Err(_) => Err("开始取流失败"),
            }
        }
        Err(e) => {
            eprintln!("获取 SDK 写锁失败: {:?}", e);
            Err("获取 SDK 写锁失败")
        }
    }
}

pub async fn stop_grabbing() -> Result<c_int, &'static str> {
    let mut sdk_guard = HC_MVS_CORE_SDK.write(); // 获取写锁
    match sdk_guard {
        Ok(mut sdk) => {
            // 调用底层 C 接口
            let result = unsafe { sdk.stop_grabbing() };
            match result {
                CResult::Ok(code) => Ok(code),
                CResult::Err(_) => Err("停止取流失败"),
            }
        }
        Err(e) => {
            eprintln!("获取 SDK 写锁失败: {:?}", e);
            Err("获取 SDK 写锁失败")
        }
    }
}

pub async fn get_image_buffer(){}

pub async fn free_image_buffer(){}

/// 提取 MvFrameOutInfoEx 中的安全字段
#[derive(Debug, Clone)]
pub struct FrameInfoSafe {
    pub nWidth: c_ushort,
    pub nHeight: c_ushort,
    pub enPixelType: MvGvspPixelType,
    pub nFrameNum: c_uint,
    pub nDevTimeStampHigh: c_uint,
    pub nDevTimeStampLow: c_uint,
    pub nHostTimeStamp: c_longlong,
    pub nFrameLen: c_uint,
    pub nSecondCount: c_uint,
    pub nCycleCount: c_uint,
    pub nCycleOffset: c_uint,
    pub fGain: c_float,
    pub fExposureTime: c_float,
    pub nAverageBrightness: c_uint,
    pub nRed: c_uint,
    pub nGreen: c_uint,
    pub nBlue: c_uint,
    pub nFrameCounter: c_uint,
    pub nTriggerIndex: c_uint,
    pub nInput: c_uint,
    pub nOutput: c_uint,
    pub nOffsetX: c_ushort,
    pub nOffsetY: c_ushort,
    pub nChunkWidth: c_ushort,
    pub nChunkHeight: c_ushort,
    pub nLostPacket: c_uint,
    pub nUnparsedChunkNum: c_uint,
    pub nReserved: [c_uint; 36], // 直接复制，不影响数据安全
}

/// 提供一个从 `MvFrameOutInfoEx` 转换为 `FrameInfoSafe` 的方法
impl From<&MvFrameOutInfoEx> for FrameInfoSafe {
    fn from(info: &MvFrameOutInfoEx) -> Self {
        Self {
            nWidth: info.nWidth,
            nHeight: info.nHeight,
            enPixelType: info.enPixelType,
            nFrameNum: info.nFrameNum,
            nDevTimeStampHigh: info.nDevTimeStampHigh,
            nDevTimeStampLow: info.nDevTimeStampLow,
            nHostTimeStamp: info.nHostTimeStamp,
            nFrameLen: info.nFrameLen,
            nSecondCount: info.nSecondCount,
            nCycleCount: info.nCycleCount,
            nCycleOffset: info.nCycleOffset,
            fGain: info.fGain,
            fExposureTime: info.fExposureTime,
            nAverageBrightness: info.nAverageBrightness,
            nRed: info.nRed,
            nGreen: info.nGreen,
            nBlue: info.nBlue,
            nFrameCounter: info.nFrameCounter,
            nTriggerIndex: info.nTriggerIndex,
            nInput: info.nInput,
            nOutput: info.nOutput,
            nOffsetX: info.nOffsetX,
            nOffsetY: info.nOffsetY,
            nChunkWidth: info.nChunkWidth,
            nChunkHeight: info.nChunkHeight,
            nLostPacket: info.nLostPacket,
            nUnparsedChunkNum: info.nUnparsedChunkNum,
            nReserved: info.nReserved,
        }
    }
}


// 回调函数的实现
extern "C" fn my_image_callback(
    pData: *const c_uchar,
    pstFrameInfo: *const MvFrameOutInfoEx,
    pUser: *const c_void,
) -> std::ffi::c_void {
    unsafe {
        if !pstFrameInfo.is_null() {
            let frame_info_safe = FrameInfoSafe::from(&*pstFrameInfo);
            let height = frame_info_safe.nHeight as u32;
            let width = frame_info_safe.nWidth as u32;
            let image_data = std::slice::from_raw_parts(pData, (height * width) as usize).to_vec();

            if let Some(tx) = GLOBAL_SENSOR_TX.lock().unwrap().as_ref() {
                let _ = tx.send(SensorsDataRequest::ImageProcess(frame_info_safe,image_data));
            }
        }
        }
    
    unsafe { std::mem::zeroed() }
}

pub async fn register_callback() -> Result<c_int, &'static str> {
    let mut sdk_guard = HC_MVS_CORE_SDK.write(); // 获取可变锁
    match sdk_guard {
        Ok(mut sdk) => {
            // 注册回调函数
            let user_data = std::ptr::null(); // 你可以传递自定义数据
            let callback_fn: CbOutputCallback = my_image_callback as CbOutputCallback;

            let result = unsafe { sdk.register_image_callback_ex(callback_fn, user_data) };
            match result {
                CResult::Ok(code) => Ok(code),
                CResult::Err(_) => Err("注册回调失败"),
            }
        }
        Err(e) => {
            eprintln!("注册回调函数失败: {:?}", e);
            Err("获取 注册回调 写锁失败") // 返回错误信息
        }
    }
}

pub async fn register_callback_rgb() -> Result<c_int, &'static str> {
    let mut sdk_guard = HC_MVS_CORE_SDK.write(); // 获取可变锁
    match sdk_guard {
        Ok(mut sdk) => {
            // 注册回调函数
            let user_data = std::ptr::null(); // 你可以传递自定义数据
            let callback_fn: CbOutputCallback = my_image_callback as CbOutputCallback;

            let result = unsafe { sdk.register_image_callback_for_rgb(callback_fn, user_data) };
            match result {
                CResult::Ok(code) => Ok(code),
                CResult::Err(_) => Err("注册回调失败"),
            }
        }
        Err(e) => {
            eprintln!("注册回调函数失败: {:?}", e);
            Err("获取 注册回调 写锁失败") // 返回错误信息
        }
    }
}

pub async fn register_callback_bgr() -> Result<c_int, &'static str> {
    let mut sdk_guard = HC_MVS_CORE_SDK.write(); // 获取可变锁
    match sdk_guard {
        Ok(mut sdk) => {
            // 注册回调函数
            let user_data = std::ptr::null(); // 你可以传递自定义数据
            let callback_fn: CbOutputCallback = my_image_callback as CbOutputCallback;

            let result = unsafe { sdk.register_image_callback_for_bgr(callback_fn, user_data) };
            match result {
                CResult::Ok(code) => Ok(code),
                CResult::Err(_) => Err("注册回调失败"),
            }
        }
        Err(e) => {
            eprintln!("注册回调函数失败: {:?}", e);
            Err("获取 注册回调 写锁失败") // 返回错误信息
        }
    }
}

pub async fn get_oneframe_timeout() -> Result<(Vec<u8>, MvFrameOutInfoEx), &'static str>{
    let mut sdk_guard = HC_MVS_CORE_SDK.read(); // 获取可变锁
    match sdk_guard {
        Ok(mut sdk) => {
            // 注册回调函数
            let mut frame_info = MvFrameOutInfoEx::default();
            let timeout: c_uint = 500; // 超时时间 1000ms
            let data_size = 5472 * 3648; // 预分配数据大小，假设最大为 BayerGB 图像
            let mut buffer = vec![0u8; data_size];

            let result = unsafe {
                sdk.get_one_frame_timeout(
                    buffer.as_mut_ptr(),
                    data_size as c_uint,
                    &mut frame_info as *mut MvFrameOutInfoEx,
                    timeout,
                )
            };

            match result {
                CResult::Ok(_) => {
                    buffer.truncate(frame_info.nFrameLen as usize);
                    Ok((buffer, frame_info))

                }
                CResult::Err(_) => Err("获取图像帧失败"),
            }
            
        }
        Err(e) => {
            eprintln!("获取图像帧失败: {:?}", e);
            Err("获取 图像帧 写锁失败") // 返回错误信息
        }
    }

}

// 用于获取设备列表
pub async fn get_device_list() -> Result<MvCcDeviceInfoList, &'static str> {
    let mut sdk_guard = HC_MVS_CORE_SDK.write(); // 获取可变锁
    match sdk_guard {
        Ok(mut sdk) => {
            // 获取设备列表副本
            let device_list = sdk.get_devlist().clone(); // 假设 MvCcDeviceInfoList 实现了 Clone
            Ok(device_list)
        }
        Err(e) => {
            eprintln!("获取设备列表失败: {:?}", e);
            Err("获取 SDK 写锁失败") // 返回错误信息
        }
    }
}

pub async fn create_handle() -> Result<Vec<i32>, &'static str> {
    // 成功时，可以访问 sdk
    let device_list = get_device_list().await?;  // 获取设备列表

    // 创建设备句柄
    let device_info_list = &device_list.pDeviceInfo;

    let mut sdk_guard = HC_MVS_CORE_SDK.write();
    match sdk_guard {
        Ok(mut sdk) => {
            
            let mut results = Vec::new();

            // 这里我们确保 sdk 的可变借用不会与不可变借用冲突
            for device_info in device_info_list {
                // 在此处调用 create_handle，确保它不与 device_list 的借用冲突
                unsafe {
                    let handle = sdk.create_handle(&**device_info);
                    match handle {
                        CResult::Ok(val) => results.push(val),    // 如果是 Ok，获取值并添加到结果中
                        CResult::Err(_) => results.push(-1),   // 如果是 Err，使用默认值 false
                    }
                }
            }
            Ok(results)
        }
        Err(e) => {
            eprintln!("调用创建设备句柄失败: {:?}", e);
            Err("获取 创建句柄 写锁失败") // 这里返回 Result 类型
        }
    }
}

// 假设 `HC_MVS_CORE_SDK` 是一个全局的写锁，
// 我们要在异步函数中调用它并销毁句柄。

pub async fn destroy_handle() -> Result<(), &'static str> {
    // 获取写锁
    let mut sdk_guard = HC_MVS_CORE_SDK.write(); // 使用 `.write().await` 获取异步写锁
    match sdk_guard {
        Ok(mut sdk) => {
            // 成功获取锁后调用 `destroy_handle` 来销毁句柄
            let result = unsafe { sdk.destroy_handle() };

            match result {
                CResult::Ok(_) => {
                    println!("设备句柄销毁成功");
                    Ok(())
                }
                CResult::Err(_) => {
                    eprintln!("设备句柄销毁失败");
                    Err("设备句柄销毁失败")
                }
            }
        }
        Err(e) => {
            eprintln!("获取创建句柄写锁失败: {:?}", e);
            Err("获取 创建句柄 写锁失败") // 锁失败时返回错误
        }
    }
}

// 设备信息列表 ~english Device Information List
#[repr(C)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MvCcDeviceInfoList2<'a> {
    /// 在线设备数量
    pub nDeviceNum: u32,
    /// 支持最多256个设备
    pub pDeviceInfo: Vec<MvCcDeviceInfo2<'a>>,
}
// 设备信息 ~english Device info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MvCcDeviceInfo2<'a> {
    /// 规范的主要版本
    pub nMajorVer: u16,
    /// 规范的次要版本
    pub nMinorVer: u16,
    /// MAC地址高位
    pub nMacAddrHigh: u32,
    /// MAC地址低位
    pub nMacAddrLow: u32,
    /// 设备传输层协议类型
    pub nTLayerType: u32,
    /// 保留字节
    pub nReserved: [u32; 4],
    /// 特定名
    pub name: Cow<'a, str>,
    /// 不同设备特有信息
    #[serde(flatten)]
    pub SpecialInfo: N19MvCCDeviceInfo3Dot0E2,
}

impl<'a> Default for MvCcDeviceInfo2<'a> {
    fn default() -> Self {
        Self {
            nMajorVer: 0,
            nMinorVer: 0,
            nMacAddrHigh: 0,
            nMacAddrLow: 0,
            nTLayerType: 0,
            nReserved: [0; 4],
            SpecialInfo: N19MvCCDeviceInfo3Dot0E2::stCamLInfo(MvCamLDeviceInfo2::default()),
            name: Cow::default(),
        }
    }
}

/// 不同设备特有信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum N19MvCCDeviceInfo3Dot0E2 {
    stGigEInfo(MvGigeDeviceInfo2),
    stUsb3VInfo(MvUsb3DeviceInfo2),
    stCamLInfo(MvCamLDeviceInfo2),
}

/// CameraLink设备信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MvCamLDeviceInfo2 {
    /// 端口号
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chPortID: [u8; INFO_MAX_BUFFER_SIZE],
    /// 设备型号
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chModelName: [u8; INFO_MAX_BUFFER_SIZE],
    /// 家族名字
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chFamilyName: [u8; INFO_MAX_BUFFER_SIZE],
    /// 设备版本号
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chDeviceVersion: [u8; INFO_MAX_BUFFER_SIZE],
    /// 制造商名字
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chManufacturerName: [u8; INFO_MAX_BUFFER_SIZE],
    /// 序列号
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chSerialNumber: [u8; INFO_MAX_BUFFER_SIZE],
    /// 保留字节
    #[serde(with = "hikvision::utils::c_uint_38_ser")]
    pub nReserved: [c_uint; 38],
}

impl Default for MvCamLDeviceInfo2 {
    fn default() -> Self {
        Self {
            chPortID: [0; INFO_MAX_BUFFER_SIZE],
            chModelName: [0; INFO_MAX_BUFFER_SIZE],
            chFamilyName: [0; INFO_MAX_BUFFER_SIZE],
            chDeviceVersion: [0; INFO_MAX_BUFFER_SIZE],
            chManufacturerName: [0; INFO_MAX_BUFFER_SIZE],
            chSerialNumber: [0; INFO_MAX_BUFFER_SIZE],
            nReserved: [0; 38],
        }
    }
}

/// USB设备信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MvUsb3DeviceInfo2 {
    /// 控制输入端点
    pub CtrlInEndPoint: u8,
    /// 控制输出端点
    pub CtrlOutEndPoint: u8,
    /// 流端点
    pub StreamEndPoint: u8,
    /// 事件端点
    pub EventEndPoint: u8,
    /// 供应商ID号
    pub idVendor: u16,
    /// 产品ID号
    pub idProduct: u16,
    /// 设备序列号
    pub nDeviceNumber: u32,
    /// 设备GUID号
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chDeviceGUID: [u8; INFO_MAX_BUFFER_SIZE],
    /// 供应商名字
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chVendorName: [u8; INFO_MAX_BUFFER_SIZE],
    /// 型号名字
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chModelName: [u8; INFO_MAX_BUFFER_SIZE],
    /// 家族名字
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chFamilyName: [u8; INFO_MAX_BUFFER_SIZE],
    /// 设备版本号
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chDeviceVersion: [u8; INFO_MAX_BUFFER_SIZE],
    /// 制造商名字
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chManufacturerName: [u8; INFO_MAX_BUFFER_SIZE],
    /// 序列号
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chSerialNumber: [u8; INFO_MAX_BUFFER_SIZE],
    /// 用户自定义名字
    #[serde(with = "hikvision::utils::c_uchar_64_ser")]
    pub chUserDefinedName: [u8; INFO_MAX_BUFFER_SIZE],
    /// 支持的USB协议
    pub nbcdUSB: u32,
    /// 设备地址
    pub nDeviceAddress: u32,
    /// 保留字节
    pub nReserved: [c_uint; 2],
}


impl Default for MvUsb3DeviceInfo2 {
    fn default() -> Self {
        Self {
            CtrlInEndPoint: 0,
            CtrlOutEndPoint: 0,
            StreamEndPoint: 0,
            EventEndPoint: 0,
            idVendor: 0,
            idProduct: 0,
            nDeviceNumber: 0,
            chDeviceGUID: [0; INFO_MAX_BUFFER_SIZE],
            chVendorName: [0; INFO_MAX_BUFFER_SIZE],
            chModelName: [0; INFO_MAX_BUFFER_SIZE],
            chFamilyName: [0; INFO_MAX_BUFFER_SIZE],
            chDeviceVersion: [0; INFO_MAX_BUFFER_SIZE],
            chManufacturerName: [0; INFO_MAX_BUFFER_SIZE],
            chSerialNumber: [0; INFO_MAX_BUFFER_SIZE],
            chUserDefinedName: [0; INFO_MAX_BUFFER_SIZE],
            nbcdUSB: 0,
            nDeviceAddress: 0,
            nReserved: [0; 2],
        }
    }
}


/// GigE设备信息
#[repr(C)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MvGigeDeviceInfo2 {
    /// IP配置选项
    pub nIpCfgOption: u32,
    /// 当前IP地址配置: bit31-static bit30-dhcp bit29-lla
    pub nIpCfgCurrent: u32,
    /// 当前主机IP地址
    pub nCurrentIp: u32,
    /// 当前子网掩码
    pub nCurrentSubNetMask: u32,
    /// 默认网关
    pub nDefaultGateWay: u32,
    /// 厂商名称
    pub chManufacturerName: [u8; 32],
    /// 型号名称
    pub chModelName: [u8; 32],
    /// 设备固件版本
    pub chDeviceVersion: [u8; 32],
    /// 厂商特殊信息
    #[serde(with = "hikvision::utils::c_uchar_48_ser")]
    pub chManufacturerSpecificInfo: [u8; 48],
    /// 序列号
    pub chSerialNumber: [u8; 16],
    /// 用户定义名称
    pub chUserDefinedName: [u8; 16],
    /// 网口Ip地址
    pub nNetExport: u32,
    /// 保留字节
    pub nReserved: [c_uint; 4],
}


impl Default for MvGigeDeviceInfo2 {
    fn default() -> Self {
        Self {
            nIpCfgOption: 0,
            nIpCfgCurrent: 0,
            nCurrentIp: 0,
            nCurrentSubNetMask: 0,
            nDefaultGateWay: 0,
            chManufacturerName: [0; 32],
            chModelName: [0; 32],
            chDeviceVersion: [0; 32],
            chManufacturerSpecificInfo: [0; 48],
            chSerialNumber: [0; 16],
            chUserDefinedName: [0; 16],
            nNetExport: 0,
            nReserved: [0; 4],
        }
    }
}
 
impl<'a> From<&mut MvCcDeviceInfoList> for MvCcDeviceInfoList2<'a> {
    fn from(value: &mut MvCcDeviceInfoList) -> Self {
        let pDeviceInfo: Vec<MvCcDeviceInfo2> = value
            .pDeviceInfo
            .iter()
            .enumerate()
            .filter_map(|(i, x)| unsafe {
                (*x).as_ref().map(|x| {
                    let special_info;
                    let name;
                    
                    let addr_fn = |x: u32| -> IpAddr {
                        IpAddr::V4(Ipv4Addr::new(
                            ((x & 0xff000000) >> 24) as u8,
                            ((x & 0x00ff0000) >> 16) as u8,
                            ((x & 0x0000ff00) >> 8) as u8,
                            (x & 0x000000ff) as u8,
                        ))
                    };
                    
                    #[allow(unused_unsafe)]
                    unsafe {
                        match &x.SpecialInfo {
                            N19MvCCDeviceInfo3Dot0E { stGigEInfo } => {
                                special_info = N19MvCCDeviceInfo3Dot0E2::stGigEInfo(MvGigeDeviceInfo2 {
                                    nIpCfgOption: stGigEInfo.nIpCfgOption,
                                    nIpCfgCurrent: stGigEInfo.nIpCfgCurrent,
                                    nCurrentIp: stGigEInfo.nCurrentIp,
                                    nCurrentSubNetMask: stGigEInfo.nCurrentSubNetMask,
                                    nDefaultGateWay: stGigEInfo.nDefaultGateWay,
                                    chManufacturerName: stGigEInfo.chManufacturerName,
                                    chModelName: stGigEInfo.chModelName,
                                    chDeviceVersion: stGigEInfo.chDeviceVersion,
                                    chManufacturerSpecificInfo: stGigEInfo.chManufacturerSpecificInfo,
                                    chSerialNumber: stGigEInfo.chSerialNumber,
                                    chUserDefinedName: stGigEInfo.chUserDefinedName,
                                    nNetExport: stGigEInfo.nNetExport,
                                    nReserved: stGigEInfo.nReserved,
                                });
                                
                                let chUserDefinedName = stGigEInfo.chUserDefinedName.to_string_without_nulls();
                                let chModelName = stGigEInfo.chModelName.to_string_without_nulls();
                                let address = addr_fn(stGigEInfo.nCurrentIp);
                                name = format!("[{i}]GigE: {chUserDefinedName} {chModelName} ({address})").into();
                            }
                            N19MvCCDeviceInfo3Dot0E { stUsb3VInfo } => {
                                special_info = N19MvCCDeviceInfo3Dot0E2::stUsb3VInfo(MvUsb3DeviceInfo2 {
                                    CtrlInEndPoint: stUsb3VInfo.CtrlInEndPoint,
                                    CtrlOutEndPoint: stUsb3VInfo.CtrlOutEndPoint,
                                    StreamEndPoint: stUsb3VInfo.StreamEndPoint,
                                    EventEndPoint: stUsb3VInfo.EventEndPoint,
                                    idVendor: stUsb3VInfo.idVendor,
                                    idProduct: stUsb3VInfo.idProduct,
                                    nDeviceNumber: stUsb3VInfo.nDeviceNumber,
                                    chDeviceGUID: stUsb3VInfo.chDeviceGUID,
                                    chVendorName: stUsb3VInfo.chVendorName,
                                    chModelName: stUsb3VInfo.chModelName,
                                    chFamilyName: stUsb3VInfo.chFamilyName,
                                    chDeviceVersion: stUsb3VInfo.chDeviceVersion,
                                    chManufacturerName: stUsb3VInfo.chManufacturerName,
                                    chSerialNumber: stUsb3VInfo.chSerialNumber,
                                    chUserDefinedName: stUsb3VInfo.chUserDefinedName,
                                    nbcdUSB: stUsb3VInfo.nbcdUSB,
                                    nDeviceAddress: stUsb3VInfo.nDeviceAddress,
                                    nReserved: stUsb3VInfo.nReserved,
                                });
                                
                                let chUserDefinedName = stUsb3VInfo.chUserDefinedName.to_string_without_nulls();
                                let chModelName = stUsb3VInfo.chModelName.to_string_without_nulls();
                                let strSerialNumber = stUsb3VInfo.chSerialNumber.to_string_without_nulls();
                                name = format!("[{i}]USB: {chUserDefinedName} {chModelName} ({strSerialNumber})").into();
                            }
                            N19MvCCDeviceInfo3Dot0E { stCamLInfo } => {
                                special_info = N19MvCCDeviceInfo3Dot0E2::stCamLInfo(MvCamLDeviceInfo2 {
                                    chPortID: stCamLInfo.chPortID,
                                    chModelName: stCamLInfo.chModelName,
                                    chFamilyName: stCamLInfo.chFamilyName,
                                    chDeviceVersion: stCamLInfo.chDeviceVersion,
                                    chManufacturerName: stCamLInfo.chManufacturerName,
                                    chSerialNumber: stCamLInfo.chSerialNumber,
                                    nReserved: stCamLInfo.nReserved,
                                });
                                
                                let chPortID = stCamLInfo.chPortID.to_string_without_nulls();
                                let chModelName = stCamLInfo.chModelName.to_string_without_nulls();
                                let strSerialNumber = stCamLInfo.chSerialNumber.to_string_without_nulls();
                                name = format!("[{i}]CameraLink: {chPortID} {chModelName} ({strSerialNumber})").into();
                            }
                        }
                    };
                    
                    MvCcDeviceInfo2 {
                        nMajorVer: x.nMajorVer,
                        nMinorVer: x.nMinorVer,
                        nMacAddrHigh: x.nMacAddrHigh,
                        nMacAddrLow: x.nMacAddrLow,
                        nTLayerType: x.nTLayerType,
                        nReserved: x.nReserved,
                        SpecialInfo: special_info,
                        name,
                    }
                })
            })
            .collect();

        Self {
            nDeviceNum: value.nDeviceNum,
            pDeviceInfo,
        }
    }
}
