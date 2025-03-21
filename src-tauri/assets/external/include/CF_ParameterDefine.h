#ifndef CF_PARAMETERDEFINE_H
#define CF_PARAMETERDEFINE_H





/**,
* describe: 启动传感器采集
* type:	    int
* channel:  忽略
* value:    0x01: 启动采集   0x00：停止采集
*/
#define  PARAM_SAMPLE_STATE                                       "sample_state"


/**,
 * describe: 传感器增益
 * type:	 int
 * channel:  忽略
 * value:	 参考 Confocal_Gain_t 定义
 */
#define  PARAM_SENSOR_GAIN                                        "sensor_gain"

/**,
 * describe: 传感器曝光时间,单位us
 * type:	 int
 * channel:  忽略
 * value:	 参考 PresetExposureTime_t 定义
 */
#define PARAM_EXPOSURE_TIME                                       "exposure_time"

/**,
* describe: 自动曝光，适用于CF3000LITE版本控制器且单个通道模式
* type:     int
* channel:  忽略
* value:    1：使能		0：关闭
*/
#define PARAM_AUTO_EXP  										    "auto_exp"

/**,
* describe: 自动曝光模式下允许的最大曝光时间，适用于CF3000LITE版本控制器且单个通道模式
* type:     int
* channel:  忽略
* value:    最大调节曝光时间，单位us
*/
#define PARAM_MAX_AUTO_EXP  										 "max_auto_exp"


/**,
* describe: 自动曝光模式下以哪一个通道为调节曝光的依据，适用于CF3000LITE版本控制器且单个通道模式，多个通道时其他通道信号会受到影响
* type:     int
* channel:  忽略
* value:    通道索引0~1
*/
#define PARAM_AUTO_EXP_CHANNEL  								      "auto_exp_channel"



 /**,
 * describe: 自动调光
 * type:	 int
 * channel:  指定传感头通道0~3
 * value:    1：使能		0：关闭
 */
#define PARAM_AUTO_LIGHT										    "auto_light"

 /**,
 * describe: 自动调光模式
 * type:	 int
 * channel:  指定传感头通道0~3 
 * value:    参考 Confocal_AutoLightMode_t
 */
#define PARAM_AUTO_LIGHT_MODE                                        "auto_light_mode"


/**,
* describe: 设置LED亮度
* type:		float
* channel:  指定传感头通道0~3
* value:    0~100
*/
#define PARAM_LIGHT_VALUE                                            "light_value"


/**,
* describe: 获取控制器连接状态
* type:	    int
* channel:  忽略
* value:    0x01：已连接   0x00：未连接
*/
#define PARAM_CONNECT_STATE                                            "connect_state"



/**,
* describe: 激活指定通道，默认所有通道都处于激活状态
* type:	    int
* channel:  指定传感头通道0~3
* value:    0x01: 激活   0x00：关闭
*/
#define  PARAM_ACTIVE_CHANNEL                                         "active_channel"



/**,
* describe: 设置控制器IP（使用于CF2000网口控制器）
* type:	    string
* channel:  忽略
* value:    IP地址
*/
#define  PARAM_CONTROLLER_IP										  "controller_ip"


/**,
* describe: 设置控制器端口（使用于CF2000网口控制器）
* type:	    int
* channel:  忽略
* value:    端口
*/
#define  PARAM_CONTROLLER_PORT										"controller_prot"


/**,
* describe: 设置控制器MAC地址（使用于CF2000网口控制器）
* type:	    string
* channel:  忽略
* value:    MAC地址
*/
#define  PARAM_CONTROLLER_MAC										"controller_mac"



/**,
* describe: SDK版本号
* type:	    string
* channel:  忽略
* value:    返回SDK版本号字符串
*/
#define PARAM_SDK_VERSION                                          "sdk_version"


/**,
* describe: 控制器版本号
* type:		string
* channel:  忽略
* value:    返回控制器版本号字符串
*/
#define PARAM_CONTROLER_VERSION                                     "controler_version"



/**,
* describe: 传感头序列号
* type:	    string
* channel:  指定传感头通道0~3
* value:    返回传感器序列号字符串
*/
#define PARAM_SENSOR_SN												 "sensor_sn"


/**,
* describe: 控制器序列号
* type:	    string
* channel:  忽略
* value:    返回控制器序列号字符串
*/
#define PARAM_CONTROLER_SN										     "controler_sn"


/**,
* describe: 帧率控制（内部定时触发模式）。使能后按照用户设定的帧率进行采集，若不使能则传感器按照当前曝光时间、通道数量能达到的最高帧率进行采集。
* type:	    int
* channel:  忽略
* value:    1：使能   0：关闭
*/
#define PARAM_FRAME_RATE_CONTROL								      "frame_rate_control"


/**,
* describe:设置传感器帧率(内部定时触发且处于帧率控制器模式下有效）
* type:	   int
* channel: 忽略
* value: 
*/
#define PARAM_FRAME_RATE											    "frame_rate"


/**,
* describe:传感器当前实时帧率
* type:	   int
* channel: 忽略
* value:   返回当前传感器的测量帧率
*/
#define PARAM_RT_FRAME_RATE											    "rt_frame_rate"



/**,
* describe: 传感器触发模式
* type:	    int
* channel:  忽略
* value:	参考 Confocal_TriggerMode_t 定义
*/
#define PARAM_TRIGGER_MODE											    "trigger_mode"


/**,
* describe:	触发同步输出信号使能，OUT0根据外部输入的触发信号，输出同步脉冲
* type:	    int
* channel:  忽略
* value:	1：使能    0：关闭
*/
#define PARAM_TRIGGER_SYNC_OUT                                          "trigger_sync_out"


/**,
* describe: 传感器编码器分频
* type:	    int
* channel:  忽略
* value:	1~65535
*/
#define PARAM_ENCODER_DIVISION										   "encoder_division"


/**,
* describe: 传感器编码器输入模式
* type:	    int
* channel:  忽略
* value:	参考 Confocal_EncoderInputMode_t 定义
*/
#define PARAM_ENCODER_INPUT_MODE									   "encoder_input_mode"


/**,
* describe: 传感器编码器工作模式
* type:	    int
* channel:  忽略
* value:	参考 Confocal_EncoderWorkingMode_t 定义
*/
#define PARAM_ENCODER_WORKING_MODE									   "encoder_working_mode"


/**,
* describe: 清零编码器分频计数值
* type:	    int
* channel:  编码器通道0~3
* value:	0
*/
#define PARAM_ENCODER_CLEAR_COUNT									   "encoder_clear_count"

/**,
* describe: 设置编码器通道
* type:	    int
* channel:  忽略
* value:	0~2
*/
#define PARAM_ENCODER_CHANNEL									       "encoder_channel"



/**,
* describe: 设置传感器滑动平均滤波窗口
* type:	    int
* channel:  指定传感头通道0~3
* value:	1~255
*/
#define PARAM_MOVING_AVG_FILTER											  "moving_avg_filter"


/**,
* describe: 清零滑动平均滤波缓存
* type:	    int
* channel:  指定传感头通道0~3
* value:	0
*/
#define PARAM_CLR_MOVING_AVG_FILTER_CNT									   "clr_moving_avg_filter_cnt"

/**,
* describe: 设置传感器滑动中值滤波窗口
* type:	    int
* channel:  指定传感头通道0~3
* value:	1~255
*/
#define PARAM_MOVING_MEDIAN_FILTER										  "moving_median_filter"


/**,
* describe: 清零滑动中值滤波缓存
* type:	    int
* channel:  指定传感头通道0~3
* value:	0
*/
#define PARAM_CLR_MOVING_MEDIAN_FILTER_CNT								  "clr_moving_median_filter_cnt"


/**,
* describe: 卡尔曼滤波
* type:	    int
* channel:  指定传感头通道0~3
* value:	1:使能   0:关闭
*/
#define PARAM_KALMAN_FILTER_EN              							  "kalman_filter_en"

/**,
* describe: 卡尔曼滤波系数K
* type:	    float
* channel:  指定传感头通道0~3
* value:	卡尔曼滤波系数K
*/
#define PARAM_KALMAN_FILTER_K              								  "kalman_filter_k"


/**,
* describe: 卡尔曼滤波阈值
* type:	    float
* channel:  指定传感头通道0~3
* value:	卡尔曼滤波阈值
*/
#define PARAM_KALMAN_FILTER_THRE             							  "kalman_filter_thre"


/**,
* describe: 卡尔曼滤波检测点数
* type:	    int
* channel:  指定传感头通道0~3
* value:	卡尔曼滤波检测点数
*/
#define PARAM_KALMAN_FILTER_NUM             							  "kalman_filter_num"


/**,
* describe: 无效数据滤波
* type:	    int
* channel:  指定传感头通道0~3
* value:	1:使能   0:关闭
*/
#define PARAM_ERROR_FILTER_EN             							      "error_filter_en"


/**,
* describe: 无效数据滤波点数
* type:	    int
* channel:  指定传感头通道0~3
* value:	无效数据滤波点数
*/
#define PARAM_ERROR_FILTER_CNT            							      "error_filter_cnt"


/**,
* describe: 选择输出的信号；用户可以选择近端信号、远端信号、强度最强的信号
* type:	    int
* channel:  指定传感头通道0~3
* value:	参考 Confocal_SignalSelect_t
*/
#define PARAM_SELECT_SIGNAL											       "select_signal"


/**,
* describe: 选择输出的信号；用户可以选择近端信号、远端信号、强度最强的信m
/**,
* describe: 获取对应通道的信号饱和度
* type:	    float
* channel:  指定传感头通道0~3
* value:	返回信号饱和度(0~100)%
*/
#define PARAM_SIGNAL_SATURATION											    "signal_saturation"


/**,
* describe: 主波峰索引
* type:	    int
* channel:  指定传感头通道0~3
* value:	主波峰索引值(0~9)
*/
#define PARAM_MAIN_SIGNAL_INDEX										       "main_signal_index"


/**,
* describe: 设置单个通道的测量模式。可以设置测距模式、测厚度模式
* type:	    int
* channel:  指定传感头通道0~3
* value:	参考 Confocal_MeasureMode_t
*/
#define PARAM_CHANNEL_MEASURE_MODE										   "channel_measure_mode"


/**,
* describe: 设置单个传感头多距离模式，输出检测到的所有信号距离值
* type:	    int
* channel:  指定传感头通道0~3
* value:	1:使能   0:关闭
*/
#define PARAM_CHANNEL_MULT_DIS									 	       "channel_mult_dis"

/**,
* describe: 设置单个传感头ABS测量模式，输出绝对测量值，不受ZERO影响
* type:	    int
* channel:  指定传感头通道0~3
* value:	1:使能   0:关闭
*/
#define PARAM_CHANNEL_ABS_MODE										 	    "channel_abs_mode"


/**,
* describe: 设置单个传感头测量值取反
* type:	    int
* channel:  指定传感头通道0~3
* value:	1:使能   0:关闭
*/
#define PARAM_CHANNEL_REVERSE_VALUE										 	"channel_revere_value"



/**,
* describe: 设置通道偏置，单位mm
* type:	    float
* channel:  指定传感头通道0~3
* value:	通道偏置
*/
#define PARAM_CHANNEL_OFFSET												 "channel_offset"


/**,
* describe: 设置信号检测阈值，为背景噪声标准差的倍数,默认值为3
* type:	    float
* channel:  指定传感头通道0~3
* value:	0.1~100.0
*/
#define PARAM_SIGNAL_DETECT_THRE										 	 "signal_detect_thre"



/**,
* describe: 设置信号计算的比例，设置为1则将信号最大值到最小值的所有点进行计算;0.5则取信号最大值往下50%的区域进行计算
* type:	    float
* channel:  指定传感头通道0~3
* value:	0.1~1.0
*/
#define PARAM_SIGNAL_CAL_RATIO    										 	 "signal_cal_ratio"


/**,
* describe: 保存Zero的零点值
* type:	    int
* channel:  指定传感头通道0~3
* value:	0x01:保存当前Zero零点值
*/
#define PARAM_SAVE_ZERO_POSITION    										 	 "save_zero_position"


/**,
* describe: 清除Zero的零点值
* type:	    int
* channel:  指定传感头通道0~3
* value:	0x01:清除当前Zero零点值
*/
#define PARAM_CLEAR_ZERO_POSITION    										 	 "clear_zero_position"


/**,
* describe: 自动信号检测
* type:	    int
* channel:  指定传感头通道0~3
* value:    1:使能   0:关闭
*/
#define PARAM_AUTO_SIGNAL_DETECT                                                 "auto_signal_detect"


/**,
* describe: 设置信检测灵敏度(1~6)（近端模式、远端模式、多距离模式下有效），灵敏度越高，越容易检测到微小的信号
* type:	    int
* channel:  指定传感头通道0~3
* value:	1~6
*/
#define PARAM_SIGNAL_DETECT_SENSITIVITY                                          "signal_detect_sensitivity"


/**,
* describe: 设置检测的信号个数，如果检测到的信号个数不等于用户设定的个数，则输出无效值
* type:	    int
* channel:  指定传感头通道0~3
* value:	1~10
*/
#define PARAM_SIGNAL_DETECT_NUM     										 "signal_detect_num"



/**,
* describe: 设置信号最小点数，信号点数小于该数值的输出无效值
* type:	    int
* channel:  指定传感头通道0~3
* value:	1~255
*/
#define PARAM_SIGNAL_MINI_POINTS     										  "signal_mini_points"


/**,
* describe: 设置信号平滑长度，默认值为8；平滑可以去掉一些信号噪毛刺
* type:	    int
* channel:  忽略
* value:	1~255
*/ 
#define PARAM_SIGNAL_SMOOTH										               "signal_smooth"



/**,
* describe: 对信号进行排序（近端到远端、远端到近端、索引从左到右）
* type:	    int
* channel:  指定传感头通道0~3
* value:	参考 Confocal_SignalSort_t
*/
#define PARAM_SIGNAL_SORT  									                   "signal_sort"



/**,
* describe: 测量结果SC_ResultDataTypeDef_t的signal字段返回信号数据的地址，用于调试
* type:	    int
* channel:  指定传感头通道0~3
* value:	1：使能   0：关闭
*/
#define PARAM_SIGNAL_DATA_OUTPUT 									            "signal_data_output"



/**,
* describe: 设置内部缓存大小；默认在X64系统下可以缓存100K个测量数据/X86系统下可以缓存8K个测量数据;用户可根据使用场景进行调整，,用户可通过CF_ExportCacheData导出Cache数据
* type:	    int
* channel:  指定传感头通道0~3
* value:	1~10000000
*/
#define PARAM_CACHE_SIZE														 "cache_size"


/**,
* describe:获取当前缓存内的数据个数,用户可通过CF_ExportCacheData导出Cache数据
* type:	    int
* channel:  指定传感头通道0~3
* value:	返回当前缓存的数据个数
*/
#define PARAM_CACHE_DATA_CNT													  "cache_data_cnt"


/**,
* describe: 清空Cache内部的数据
* type:	    int
* channel:  指定传感头通道0~3
* value:	0
*/
#define PARAM_CACHE_CLEAR													     "cache_clear"


/**,
* describe: 设置Cache缓存阈值，数据量达到阈值后通过回调函数通知用户,用户可通过CF_ExportCacheData导出Cache数据
* type:	    int
* channel:  指定传感头通道0~3
* value:	1~10000000
*/
#define PARAM_CACHE_THRE  													      "cache_thre"

/**,
* describe: 设置模拟量输出
* type:	    int
* channel:  忽略
* value:	1：使能		0：关闭
*/
#define PARAM_ANALOG_VOL_OUTPUT  												   "analog_vol_output"



/**,
* describe: 设置RSXXX输出协议:可选择RS485\RS232\RS422输出
* type:	    int
* channel:  忽略
* value:	参考 COMM_Protocol_Enum_t
*/
#define PARAM_RSXXX_PROTOCOL 												       "rsxxx_protocol"


/**,
* describe: 设置RSXXX波特率
* type:	    int
* channel:  忽略
* value:	参考 COMM_BaudRate_Enum_t
*/
#define PARAM_RSXXX_BAUDRATE												       "rsxxx_baudrate"



/**,
* describe: 设置RSXXX奇偶校验
* type:	    int
* channel:  忽略
* value:	参考 COMM_Parity_Enum_t
*/	
#define PARAM_RSXXX_PARITY															"rsxxx_parity"




/**,
* describe: 设置RSXXX数据解析个数，Hex模式或ASCII模式
* type:	    int
* channel:  忽略
* value:	参考 COMM_Data_Format_Enum_t
*/
#define PARAM_RSXXX_DATA_FORMAT														"rsxxx_data_format"


/**,
* describe: 设置RSXXX协议选择方式，外部硬件IO选择或软件选择,默认通过SDK选择
* type:	    int
* channel:  忽略
* value:	参考 COMM_Protocol_Control_Enum_t
*/
#define PARAM_RSXXX_SELECT															"rsxxx_select"




/**,
* describe: 设置双头测厚模式
* type:	    int
* channel:  忽略
* value:	1：使能    0：关闭
*/
#define PARAM_DOUBLE_CHANNEL_MODE													 "double_channel_mode"


/**,
* describe: 双头测厚模式下，设置交替光源切换，用于测量半透明物体
* type:	    int
* channel:  忽略
* value:	1：使能    0：关闭
*/
#define PARAM_DOUBLE_CHANNEL_AL_MODE												  "double_channel_al_mode"


/**,
* describe: 双头测厚模式下，翻转两个传感头的测量值
* type:	    int
* channel:  0~1  0:对应传感头0/1通道组成的双头测厚    1:对应传感头2/3通道组成的双头测厚
* value:	1：使能    0：关闭
*/
#define PARAM_DOUBLE_CHANNEL_REVERSE_DIS											   "double_channel_reverse_dis"


/**,
* describe: 双头测厚模式下，设置标定模式；
*		    在标定模式下用户需要使用标准厚度的测量物体对双头测量结果进行标定
*			在非标定模式下，用户不需要使用标准厚度物体进行标定，在使用前需要将两个传感头光轴对准，然后调用CF_Zero进行归零
* type:	    int
* channel:  0~1  0:对应传感头0/1通道组成的双头测厚    1:对应传感头2/3通道组成的双头测厚
* value:	1：使能    0：关闭
*/
#define PARAM_DOUBLE_CHANNEL_CAL_MODE											       "double_channel_cal_mode"


/**,
* describe: 双头测厚标定系数a
* type:	    double
* channel:  0~1  0:对应传感头0/1通道组成的双头测厚    1:对应传感头2/3通道组成的双头测厚
* value:    标定系数a
*/
#define PARAM_DOUBLE_CHANNEL_CAL_COEF_A											        "double_channel_cal_coef_a"


/**,
* describe: 双头测厚标定系数b
* type:	    double
* channel:  0~1  0:对应传感头0/1通道组成的双头测厚    1:对应传感头2/3通道组成的双头测厚
* value:    标定系数b
*/
#define PARAM_DOUBLE_CHANNEL_CAL_COEF_B											        "double_channel_cal_coef_b"


/**,
* describe: 双头测厚标定系数c
* type:	    double
* channel:  0~1  0:对应传感头0/1通道组成的双头测厚    1:对应传感头2/3通道组成的双头测厚
* value:    标定系数c
*/
#define PARAM_DOUBLE_CHANNEL_CAL_COEF_C											        "double_channel_cal_coef_c"


/**,
* describe: 使能trigger pass 调试功能,成功使能后返回的结果里面 bTriggerPass变量用于指示是否trigger pass
* type:	    int
* channel:  忽略
* value:	1：使能    0：关闭
*/
#define PARAM_TRIGGER_PASS_DEBUG    											        "trigger_pass_debug"

/**,
* describe:	清除trigger pass 标志位
* type:	    int
* channel:  忽略
* value:	0
*/
#define PARAM_CLEAR_TRIGGER_PASS_FLAG    											      "clear_trigger_pass_flag"

/**,
* describe:	是否消除背景光
* type:	    int
* channel:  忽略
* value:	1：使能    0：关闭
*/
#define PARAM_SUB_BACKGROUND_LIGHT                                                         "sub_background_light"



/**,
* describe:	设置外部IO口IN0的功能
* type:	    int
* channel:  该IO口关联的传感器通道
* value:	Confocal_InputPortFunc_t
*/
#define PARAM_SYNC_IN0_FUNC                                                                  "sync_in0_func"


/**,
* describe:	设置外部IO口IN1的功能
* type:	    int
* channel:  该IO口关联的传感器通道
* value:	Confocal_InputPortFunc_t
*/
#define PARAM_SYNC_IN1_FUNC                                                                   "sync_in1_func"



/**,
* describe:	外部输入触发消抖时间
* type:	    int
* channel:  忽略
* value:	消抖时间，单位ms
*/
#define PARAM_EX_TRIGGER_DEBOUNCE                                                              "ex_trigger_debounce"


#endif
