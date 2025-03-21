#ifndef __CF_USERINTERFACE_H__
#define __CF_USERINTERFACE_H__

///////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Copyright (c) 2017-2023 Hypersen Technology Co. Ltd.						//
// All rights reserved.                                                     //
//                                                                          //
// THIS CODE AND INFORMATION ARE PROVIDED "AS IS" WITHOUT WARRANTY OF ANY   //
// KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE      //
// IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A               //
// PARTICULAR PURPOSE.                                                      //
//                                                                          //
// Website: www.hypersen.com                                                //
// Email: sales@hypersen.com												//
//////////////////////////////////////////////////////////////////////////////

#ifdef __cplusplus
extern "C" {
#endif


#include <stdint.h>
#include "TypeDefine.h"


#ifndef _DLL_API
#define _DLL_API  _declspec(dllexport)
#else
#define _DLL_API  _declspec(dllimport)
#endif


/**********************************************************************************
* CF_ScanDeviceList
*  扫描传感器设备列表,适用于USB3.0系列控制器
* INPUT:
*	devList:		返回当前所有设备
*	deviceNumber:	返回设备个数
*	RETURN:			返回错误码
**********************************************************************************/
_DLL_API StatusTypeDef CF_ScanDeviceList(DeviceInfo_t** devList, int* deviceNumber);

/**********************************************************************************
* CF_OpenDevice
*	打开指定的光谱仪设备,适用于USB3.0系列控制器
* INPUT:
*	device:			用户指定的传感器设备
*	deviceHandler:	返回该设备的句柄
*   mode:           DeviceType_t设备型号
*	RETURN:			返回错误码
*	
**********************************************************************************/
_DLL_API StatusTypeDef  CF_OpenDevice(DeviceInfo_t* device, DeviceHandle_t*deviceHandler, DeviceType_t model);


/**********************************************************************************
* CF_GE_openDevice
*	打开指定的光谱仪设备,适用于以太网系列控制器
* INPUT:
*	controllerPara: 控制器以太网通信参数，如果设置为NULL则使用控制器固定的通信参数进行连接
*   localIP:        地IP，如果设置为NULL则使用INADDR_ANY绑定本机的所有IP
*	deviceHandler:  返回该设备的句柄
*   mode:           DeviceType_t设备型号
*   RETURN:         返回错误码
*	
**********************************************************************************/
_DLL_API StatusTypeDef CF_GE_OpenDevice(ControllerGEPara_t* controllerPara, char* localIP, DeviceHandle_t*deviceHandler, DeviceType_t model);


/**********************************************************************************
* CF_SetFactoryFilePath
*	设置出厂配置文件搜索路径，适用于CF3000Lite版本控制器;不设置默认在程序运行目录下搜索
* INPUT:
*	path:出厂配置文件搜索路径
**********************************************************************************/
_DLL_API void CF_SetFactoryFilePath(const char* path);


/**********************************************************************************
* closeDevice
*	关闭指定的传感器设备
* INPUT:
*	handle:	用户指定的传感器设备handle
*   RETURN: 返回错误码
*
**********************************************************************************/
_DLL_API void CF_CloseDevice(DeviceHandle_t handle);


/**********************************************************************************
* CF_StartSample
*	启动传感器采集,采集的测量结果通过回调函数返回或通过调用CF_GetLatestResult/CF_GetLatestResult_MC获取最新的测量结果
* INPUT:
*	handle:		  用户指定的传感器设备handle
*   en：			  true:启动采集      false:停止采集
*	RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_StartSample(DeviceHandle_t handle, bool en);

/**********************************************************************************
* CF_Zero
*	对传感器测量值进行归零
* INPUT:
*	handle:		  用户指定的传感器设备handle
*   channelIndex：通道索引。单头模式下，该索引取值为0~3，对应4个通道传感头；在双头测厚度非标定模式下，该索引取值为0~1，对应两组双头测量
*	RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_Zero(DeviceHandle_t handle, int channelIndex);



/**********************************************************************************
* CF_RegisterEventCallback
*	注册总的事件回调函数，连续采集的测量结果/函数调用异常信息通过回调函数通知
* INPUT:
*	eventHandler:回调函数
*	userPara    :用户数据
*   RETURN:
*
**********************************************************************************/
_DLL_API void CF_RegisterEventCallback(UserEventCallbackHandle eventHandler, void*userPara);


/**********************************************************************************
* CF_GetLatestResult
*	获取一帧最新的测量值
* INPUT:
*	handle:		用户指定的传感器设备handle
*	res:		返回所有已激活通达的测量值
*	len:		返回测量结果个数
*   RETURN:		返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_GetLatestResult(DeviceHandle_t handle, SC_ResultDataTypeDef_t res[], int*len);


/**********************************************************************************
* CF_GetLatestResult_MC
*	双头测厚模式下获取一帧最新的测量值
* INPUT:
*	handle:	用户指定的传感器设备handle
*	res:	返回所有已激活通达的测量值
*	len:	返回测量结果个数
*   RETURN:	返回错误码
*	
**********************************************************************************/
_DLL_API StatusTypeDef CF_GetLatestResult_MC(DeviceHandle_t handle, MC_ResultDataTypeDef_t res[], int*len);


/**********************************************************************************
* CF_DarkSignal
*	消除传感器背景光信号,传感器启动测量前需要先消除背景光信号（将传感器移动到量程外，执行dark操作）。默认出厂已经对传感器执行dark操作，并保存到控制器中。
* INPUT:
*	handle:        用户指定的传感器设备handle
*	channel:       通道,小于0则消除所有通道的dark信号
*   presetExpTime: true:消除所有预设曝光时间的dark信号,并将数据保存到运行目录,下次运行程序无需进行dark操作；
*                  false:采集当前曝光时间的dark信号，dark数据在SDK断开连接后失效，下次连接SDK需要重新进行dark
*   RETURN: 返回错误码
**********************************************************************************/
_DLL_API StatusTypeDef CF_DarkSignal(DeviceHandle_t handle, int channel, bool presetExpTime);


/**********************************************************************************
* CF_ExportCacheData
*	将Cache里面的所有数据都获取出来，用户通过参数PARAM_CACHE_DATA_CNT获取当前Cache里面有多少个数据，在开辟好内存空间将数据获取出去;
*   通过参数 PARAM_CACHE_CLEAR 可以将Cacnhe内部数据清空
* INPUT:
*	handle		:用户指定的传感器设备handle
*	cacheIndex: ：缓存的索引(0~3)对应4个通道
*	data		: 返回的数据
*   maxDataCount: 读取的最大长度
*	dataCount	：返回的实际读取的数据长度
*   RETURN: 返回错误码
**********************************************************************************/
_DLL_API StatusTypeDef CF_ExportCacheData(DeviceHandle_t handle, int cacheIndex, double retData[], int maxCount, int32_t *dataCount);



/**********************************************************************************
* CF_SaveSetting
*	保存当前用户配置到控制器中
* INPUT:
*	handle:		  用户指定的传感器设备handle
* RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_SaveSetting(DeviceHandle_t handle);


/**********************************************************************************
* CF_RestoreFactorySetting
*	恢复出厂配置
* INPUT:
*	handle:		  用户指定的传感器设备handle
*  RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_RestoreFactorySetting(DeviceHandle_t handle);



/**********************************************************************************
* CF_ExportUserSetting
*	导出传感器配置文件
* INPUT:
*	handle:		  用户指定的传感器设备handle
*	path：		  导出路径
*   RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_ExportUserSetting(DeviceHandle_t handle, const char*path);


/**********************************************************************************
* CF_ImportUserSetting
*	导入传感器配置文件
* INPUT:
*	handle:		  用户指定的传感器设备handle
*	path：		  配置文件名
*   RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_ImportUserSetting(DeviceHandle_t handle, const char*pathName);



/**********************************************************************************
* CF_SetIntParam
*	设置Int类型参数
* INPUT:
*	handle:		  用户指定的传感器设备handle
*	paramName：	  参数名， 参数名，包含的参数可以参考CF_ParamterDefine.h文件
*   channelIndex：通道索引，若是全局参数则该值填0即可，内部不做判断
*   RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_SetIntParam(DeviceHandle_t handle,const char* paramName,int channelIndex,int value);



/**********************************************************************************
* CF_SetFloatParam
*	设置Float类型参数
* INPUT:
*	handle:		  用户指定的传感器设备handle
*	paramName：	  参数名， 参数名，包含的参数可以参考CF_ParamterDefine.h文件
*   channelIndex：通道索引，若是全局参数则该值填0即可，内部不做判断
*   RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_SetFloatParam(DeviceHandle_t handle, const char* paramName, int channelIndex, float value);

/**********************************************************************************
* CF_SetStringParam
*	设置String类型参数
* INPUT:
*	handle:		  用户指定的传感器设备handle
*	paramName：	  参数名， 参数名，包含的参数可以参考CF_ParamterDefine.h文件
*   channelIndex：通道索引，若是全局参数则该值填0即可，内部不做判断
*	RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_SetStringParam(DeviceHandle_t handle, const char* paramName, int channelIndex, char* value);


/**********************************************************************************
* CF_GetIntParam
*	获取Int类型参数值
* INPUT:
*	handle:		  用户指定的传感器设备handle
*	paramName：	  参数名， 参数名，包含的参数可以参考CF_ParamterDefine.h文件
*   channelIndex：通道索引，若是全局参数则该值填0即可，内部不做判断
*	RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_GetIntParam(DeviceHandle_t handle, const char* paramName, int channelIndex, int* value);

/**********************************************************************************
* CF_GetFloatParam
*	获取Float类型参数值
* INPUT:
*	handle:		  用户指定的传感器设备handle
*	paramName：	  参数名， 参数名，包含的参数可以参考CF_ParamterDefine.h文件
*   channelIndex：通道索引，若是全局参数则该值填0即可，内部不做判断
*   RETURN: 返回错误码
**********************************************************************************/
_DLL_API StatusTypeDef CF_GetFloatParam(DeviceHandle_t handle, const char* paramName, int channelIndex, float *value);

/**********************************************************************************
* CF_GetStringParam
*	获取String类型参数值
* INPUT:
*	handle:		  用户指定的传感器设备handle
*	paramName：	  参数名， 参数名，包含的参数可以参考CF_ParamterDefine.h文件
*   channelIndex：通道索引，若是全局参数则该值填0即可，内部不做判断
*   RETURN:		  返回错误码
*
**********************************************************************************/
_DLL_API StatusTypeDef CF_GetStringParam(DeviceHandle_t handle, const char* paramName, int channelIndex, char* value);


/**********************************************************************************
* CF_BindInputPort
*	设置外部触发IO的功能
* INPUT:
*	handle:						用户指定的传感器设备handle
*	Confocal_InputPortFunc_t:	用户指定触发功能
*	Channel：					该IO口关联的通道
*   inputPort:					输入口
*   RETURN:						返回错误码
*	
**********************************************************************************/
_DLL_API StatusTypeDef CF_BindInputPort(DeviceHandle_t handle, int Channel, Confocal_InputPortFunc_t func, int inputPort);


/**********************************************************************************
* CF_UnbindInputPort
*	将指定输入IO口解绑
* INPUT:
*  handle:				用户指定的传感器设备handle
*  inputPort:			输入口
*  RETURN:			    返回错误码
*		
**********************************************************************************/
_DLL_API StatusTypeDef CF_UnbindInputPort(DeviceHandle_t handle, int inputPort);






#ifdef __cplusplus
}
#endif

#endif
