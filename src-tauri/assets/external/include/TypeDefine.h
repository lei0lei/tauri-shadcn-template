#ifndef __TYPE_DEFINE_H__
#define __TYPE_DEFINE_H__

///////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Copyright (c) 2017-2021 Hypersen Technology Co. Ltd.						//
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

//超出量程或没信号时输出的无效值
#define	 INVALID_VALUE		888888
#define  CM_MAX_GROUP		2


//设备句柄
typedef	 int				DeviceHandle_t;

//异步通知事件类型
typedef enum
{
    EventType_DataRecv = 0,				//事件类型:接收数据
}EventTypeDef;

//返回过程数据的RID
typedef enum
{
    RID_RESULT = 0,						//测量结果
    RID_IO_BOARD_TEMP_OVERLOAD,			//温度异常
    RID_IO_BOARD_FAN_ERROR,				//风扇停转
    RID_TOLERANCE_ERROR,				//公差异常
    RID_SIGNAL_ERROR,					//信号异常
    RID_DEVICE_DISCONNECT,				//设备断开连接
    RID_API_CALL_EXCEPTION,				//API调用异常
    RID_ENCODER_COUNT,					//编码器计数值
    RID_CACHE_REACH_THRES,				//Cache缓存数据达到阈值
    RID_IO_ASYNC_EVENT,					//控制器IO口异步事件
}ConfocalDataRid_t;


//错误码定义
typedef enum
{
    Status_Succeed = 0,
    Status_Others = -1,
    Status_Offline = -2,
    Status_NoDevice =-3,
    Status_DeviceAlreadyOpen = -4,
    Status_DeviceNumberExceedLimit =-5,
    Status_OpenDeviceFailed =-6,
    Status_InvalidPara = -7,
    Status_Timeout = -8,
    Status_DeviceNotFound = -9,
    Status_NotStart = -10,
    Status_InvalidState = -11,
    Status_OutOfRange = -12,
    Status_ParaNotExist = -13,
    Status_NoSignal		= -14,
    Status_FileNotFound = -15,
    Status_NoLicense = -16,
    Status_LicenseExpired = -17,
    Status_LoadLibFailed = -18,
    Status_EnvCheckError = -19,
    Status_ErrorSDKVersion = -20,
    Status_NoParaMatch = -21,
    Status_ReadOnlyParam = -22,
    Status_HardwareNotSupported =-23
}StatusTypeDef;


//预设的积分时间,单位us
typedef enum
{
    ExposureTime_20 = 20,
    ExposureTime_50 = 50,
    ExposureTime_100 = 100,
    ExposureTime_200 = 200,
    ExposureTime_400 = 400,
    ExposureTime_700 = 700,
    ExposureTime_1000 = 1000,
    ExposureTime_1500 = 1500,
}PresetExposureTime_t;


//触发模式选择
typedef enum
{
    Trigger_Internal = 0,			//内部连续触发
    Trigger_Reserve,
    Trigger_Encoder,				//编码器触发
    Trigger_Timing,					//内部定时触发
    Trigger_SingleShot,			    //内部单次触发
}Confocal_TriggerMode_t;


//编码器输入模式
typedef enum
{
    Mode_1_INC_1 = 0,				//一相一递增
    Mode_2_INC_1 = 1,			    //两相一递增
    Mode_2_INC_2 = 2,				//两相两递增
    Mode_2_INC_4 = 3,				//两相四递增
}Confocal_EncoderInputMode_t;


//编码器工作模式
typedef enum
{
    Mode_Three_Signal_End = 0,   //三个单端
    Mode_Diff_One_Signal_End = 1,//一个差分一个单端
}Confocal_EncoderWorkingMode_t;


//外部触发源
typedef enum
{
    Sync_In_0 = 0,					//外部触发源0

}Confocal_ExternTriggerSource_t;

//外部触发功能
typedef enum
{
    Trigger_DirectCapture = 0,		//直接触发
    Trigger_CacheCapture,			//缓存触发
    Trigger_Zero,					//测量值清零

}Confocal_ExtTriggerFunc_t;


//自动调光模式
typedef enum
{
    AutoLight_MaxIntensity = 0,		//按最强的波峰调光
    AutoLight_WeakIntensity,		//按最弱的波峰进行调光
}Confocal_AutoLightMode_t;


//警报类型
typedef enum
{
    AlarmType_None,					//无报警
    AlarmType_UpperLimit,			//上公差报警
    AlarmType_LowerLimit,			//下公差报警
    AlarmTyp_DeviceDisconnect,		//设备断开报警
    AlarmType_SignalWeak,			//信号弱报警
    AlarmType_SignalSaturated,		//信号饱和报警
    AlarmType_TempError,			//温度异常报警
    AlarmType_FanError,				//风扇停转报警
}AlarmType_t;


//输入口触发功能
// changed by lei
typedef enum
{
    InputPort_None = 0,
    InputPort_ExtTrigger,			//外部触发
    InputPort_ExtTriggerCache,		//外部触发,使用Cache数据
    InputPort_Zero,					//对选定通道进行zero
    InputPort_StartSample,			//启动采样
    InputPort_StopSample,			//停止采样
    InputPort_SampleToggle,			//采样状态翻转
    InputPort_ClearCache,			//清空内部缓存
    InputPort_EnableCache,			//使能Cache缓存
    InputPort_DisableCache,			//关闭Cache缓存
    InputPort_R_Start_F_Stop,		//上升沿启动采集，下降沿停止采集
    InputPort_R_Stop_F_Start,		//上升沿停止采集，下降沿开始采集
    InputPort_RF_Async_Notice,		//IO口双边沿回调函数异步通知，SDK内部不做任何处理
    InputPort_Async_Notice		    //IO口上升沿回调函数异步通知，SDK内部不做任何处理
}Confocal_InputPortFunc_t;


//错误警报时IO开关状态
typedef enum
{
    PortState_Off = 0,				//IO断开
    PortState_On = 1				//IO闭合
}IoPortState_t;


//测量单位
typedef enum
{
    MeasuretUnit_mm = 0,			//毫米
    MeasuretUnit_um,				//微妙
    MeasuretUnit_inch,				//英寸
}Confocal_MeasuretUnit_t;

//通道测量模式
typedef enum
{
    MeasureMode_Distance = 0,		//距离模式
    MeasureMode_Thickness,			//厚度模式
}Confocal_MeasureMode_t;

//模拟增益
typedef enum
{
    Gain_1 = 1,
    Gain_2,
    Gain_3,
    Gain_4

}Confocal_Gain_t;


// 多通道协同测量下的测量模式
typedef enum
{
    CM_Thickness = 0,				//双头测厚

}Confocal_CooperationMeasureMode_t;


//单距离模式下，选择哪个信号用于计算
typedef enum
{
    Signal_MaxIntensity,			//光强最强的信号
    Signal_NearEnd,					//最近端信号
    Signal_FarEnd					//最远端信号
}Confocal_SignalSelect_t;


//多波峰模式下，信号排序
typedef enum
{

    Signal_Sort_Index = 0,			 //按CMOS索引，从左到右排序
    Signal_Sort_Near_To_Far,		 //按近端到远端排序
    Signal_Sort_Far_To_Near,		 //按远端到近端排序

}Confocal_SignalSort_t;


//通讯协议控制权
typedef enum
{
    Hardware = 0,//默认模式
    Software = 1
}COMM_Protocol_Control_Enum_t;


//选择通讯协议
typedef enum
{
    RS422_COMM = 0,					//默认模式
    RS485_COMM = 1,
    RS232_COMM = 3					//2和3都是RS232通讯
}COMM_Protocol_Enum_t;

//通讯波特率
typedef enum
{
    BaudRate_9600 = 0,
    BaudRate_19200 = 1,
    BaudRate_38400 = 2,
    BaudRate_57600 = 3,
    BaudRate_115200 = 4,		 //默认波特率
    BaudRate_230400 = 5,
    BaudRate_460800 = 6,
    BaudRate_921600 = 7,
    BaudRate_Max_Num			 //多少种波特率选择
}COMM_BaudRate_Enum_t;


//通讯的校验位,  数据格式：1bit起始位 + 8bit数据位 + 【校验位】 + 1bit停止位
typedef enum
{
    Even = 0, //设置奇偶校验位，以便设置了位的计数为偶数
    Odd = 1,  //设置奇偶校验位，以便设置了位的计数为奇数
    Mark = 2, //将奇偶校验位设置为 1
    Space = 3,//将奇偶校验位设置为 0
    None = 7  //4~7：没有奇偶校验检查时发生
}COMM_Parity_Enum_t;

//通讯的数据格式
typedef enum
{
    ASCII = 0,
    Hexadecimal = 1 //默认数据格式

}COMM_Data_Format_Enum_t;



//数据集的属性
typedef enum
{
    Attribute_MinMax,  //最大最小值
    Attribute_Avg,	   //平均值
    Attribute_PtP,	   //峰峰值
    Attribute_STD	   //标准差

}DataSetAttribute_t;

//设备型号
typedef enum
{
    HPS_CF2000,
    HPS_CF3000,
    HPS_CF4000,
    HPS_CF3000Lite
}DeviceType_t;


//独立模式下返回的结果
typedef struct
{
    int		 channelIndex;				//独立模式下该结果对应的通道
    float	 saturation;				//信号饱和度
    int		 resultLen;					//结果的个数，若没使能多距离测量或侧厚度模式，则长度为1，结果存放在result[0]中
    float    result[10];				//最多存放10个最终计算结果，根据不同的计算模式，该结果可能是距离值或厚度值
    int      distanceNumber;			//厚度模式下波峰的个数
    float    distance[20];				//存放厚度模式下每个波峰对应的距离值
    uint64_t signal;					//返回信号指针，仅内部调试有用
    int		 signalLength;				//信号长度
    int32_t  triggerCount;				//编码器通道0触发计数,只适用于CF2000控制器的编码器触发模式
    int32_t  triggerCount1;				//编码器通道1触发计数,只适用于CF2000控制器的编码器触发模式
    int32_t  triggerCount2;				//编码器通道2触发计数,只适用于CF2000控制器的编码器触发模式
    int32_t  bTriggerPass;				//用于指示是否tirgger pass，必须开启trigger pass 调试功能该变量才有效
}SC_ResultDataTypeDef_t;


//多传感头协同工作下返回的结果
typedef struct
{
    int						 groupIndex;				//组索引
    SC_ResultDataTypeDef_t	 channelResult[4];		    //每个通道单独的计算结果，在双头测量模式下，只使用前面两个数据
    float					 thickness;					//双头测厚模式下的计算结果
    int					     resultLen;
    int32_t  triggerCount;						        //编码器通道0触发计数,只适用于CF2000控制器的编码器触发模式
    int32_t  triggerCount1;							    //编码器通道1触发计数,只适用于CF2000控制器的编码器触发模式
    int32_t  triggerCount2;								//编码器通道2触发计数,只适用于CF2000控制器的编码器触发模式
    int32_t  bTriggerPass;								//用于指示是否tirgger pass，必须开启trigger pass 调试功能该变量才有效
}MC_ResultDataTypeDef_t;

//异步事件参数
typedef struct
{
    EventTypeDef		eventType;		//事件类型
    void				*data;			//数据
    int					dataLen;		//数据个数
    int					rid;			//数据RID
}EventCallbackArgs_t;

//版本信息
typedef struct
{
    unsigned char year;
    unsigned char month;
    unsigned char day;
    unsigned char major;
    unsigned char minor;
    unsigned char rev;
}version_t;


//卡尔曼滤波参数
typedef struct
{
    float            kalman_k;
    float            kalman_threshold;
    uint32_t         num_check;
}KalmanFilterPara_t;



//设备描述信息
typedef struct
{
    int serverIndex;
    char descriptor[64];
}DeviceInfo_t;


//控制器以太网连接参数
typedef struct
{
    char	  *controllerIp;
    char	  *controllerMAC;
    uint16_t  controllerPort;

}ControllerGEPara_t;

//异步通知回调函数
typedef void(*UserEventCallbackHandle)(DeviceHandle_t handler, EventCallbackArgs_t arg, void*userPara);

#ifdef __cplusplus
}
#endif


#endif
