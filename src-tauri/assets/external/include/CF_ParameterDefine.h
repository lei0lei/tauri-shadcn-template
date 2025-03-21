#ifndef CF_PARAMETERDEFINE_H
#define CF_PARAMETERDEFINE_H





/**,
* describe: �����������ɼ�
* type:	    int
* channel:  ����
* value:    0x01: �����ɼ�   0x00��ֹͣ�ɼ�
*/
#define  PARAM_SAMPLE_STATE                                       "sample_state"


/**,
 * describe: ����������
 * type:	 int
 * channel:  ����
 * value:	 �ο� Confocal_Gain_t ����
 */
#define  PARAM_SENSOR_GAIN                                        "sensor_gain"

/**,
 * describe: �������ع�ʱ��,��λus
 * type:	 int
 * channel:  ����
 * value:	 �ο� PresetExposureTime_t ����
 */
#define PARAM_EXPOSURE_TIME                                       "exposure_time"

/**,
* describe: �Զ��ع⣬������CF3000LITE�汾�������ҵ���ͨ��ģʽ
* type:     int
* channel:  ����
* value:    1��ʹ��		0���ر�
*/
#define PARAM_AUTO_EXP  										    "auto_exp"

/**,
* describe: �Զ��ع�ģʽ�����������ع�ʱ�䣬������CF3000LITE�汾�������ҵ���ͨ��ģʽ
* type:     int
* channel:  ����
* value:    �������ع�ʱ�䣬��λus
*/
#define PARAM_MAX_AUTO_EXP  										 "max_auto_exp"


/**,
* describe: �Զ��ع�ģʽ������һ��ͨ��Ϊ�����ع�����ݣ�������CF3000LITE�汾�������ҵ���ͨ��ģʽ�����ͨ��ʱ����ͨ���źŻ��ܵ�Ӱ��
* type:     int
* channel:  ����
* value:    ͨ������0~1
*/
#define PARAM_AUTO_EXP_CHANNEL  								      "auto_exp_channel"



 /**,
 * describe: �Զ�����
 * type:	 int
 * channel:  ָ������ͷͨ��0~3
 * value:    1��ʹ��		0���ر�
 */
#define PARAM_AUTO_LIGHT										    "auto_light"

 /**,
 * describe: �Զ�����ģʽ
 * type:	 int
 * channel:  ָ������ͷͨ��0~3 
 * value:    �ο� Confocal_AutoLightMode_t
 */
#define PARAM_AUTO_LIGHT_MODE                                        "auto_light_mode"


/**,
* describe: ����LED����
* type:		float
* channel:  ָ������ͷͨ��0~3
* value:    0~100
*/
#define PARAM_LIGHT_VALUE                                            "light_value"


/**,
* describe: ��ȡ����������״̬
* type:	    int
* channel:  ����
* value:    0x01��������   0x00��δ����
*/
#define PARAM_CONNECT_STATE                                            "connect_state"



/**,
* describe: ����ָ��ͨ����Ĭ������ͨ�������ڼ���״̬
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:    0x01: ����   0x00���ر�
*/
#define  PARAM_ACTIVE_CHANNEL                                         "active_channel"



/**,
* describe: ���ÿ�����IP��ʹ����CF2000���ڿ�������
* type:	    string
* channel:  ����
* value:    IP��ַ
*/
#define  PARAM_CONTROLLER_IP										  "controller_ip"


/**,
* describe: ���ÿ������˿ڣ�ʹ����CF2000���ڿ�������
* type:	    int
* channel:  ����
* value:    �˿�
*/
#define  PARAM_CONTROLLER_PORT										"controller_prot"


/**,
* describe: ���ÿ�����MAC��ַ��ʹ����CF2000���ڿ�������
* type:	    string
* channel:  ����
* value:    MAC��ַ
*/
#define  PARAM_CONTROLLER_MAC										"controller_mac"



/**,
* describe: SDK�汾��
* type:	    string
* channel:  ����
* value:    ����SDK�汾���ַ���
*/
#define PARAM_SDK_VERSION                                          "sdk_version"


/**,
* describe: �������汾��
* type:		string
* channel:  ����
* value:    ���ؿ������汾���ַ���
*/
#define PARAM_CONTROLER_VERSION                                     "controler_version"



/**,
* describe: ����ͷ���к�
* type:	    string
* channel:  ָ������ͷͨ��0~3
* value:    ���ش��������к��ַ���
*/
#define PARAM_SENSOR_SN												 "sensor_sn"


/**,
* describe: ���������к�
* type:	    string
* channel:  ����
* value:    ���ؿ��������к��ַ���
*/
#define PARAM_CONTROLER_SN										     "controler_sn"


/**,
* describe: ֡�ʿ��ƣ��ڲ���ʱ����ģʽ����ʹ�ܺ����û��趨��֡�ʽ��вɼ�������ʹ���򴫸������յ�ǰ�ع�ʱ�䡢ͨ�������ܴﵽ�����֡�ʽ��вɼ���
* type:	    int
* channel:  ����
* value:    1��ʹ��   0���ر�
*/
#define PARAM_FRAME_RATE_CONTROL								      "frame_rate_control"


/**,
* describe:���ô�����֡��(�ڲ���ʱ�����Ҵ���֡�ʿ�����ģʽ����Ч��
* type:	   int
* channel: ����
* value: 
*/
#define PARAM_FRAME_RATE											    "frame_rate"


/**,
* describe:��������ǰʵʱ֡��
* type:	   int
* channel: ����
* value:   ���ص�ǰ�������Ĳ���֡��
*/
#define PARAM_RT_FRAME_RATE											    "rt_frame_rate"



/**,
* describe: ����������ģʽ
* type:	    int
* channel:  ����
* value:	�ο� Confocal_TriggerMode_t ����
*/
#define PARAM_TRIGGER_MODE											    "trigger_mode"


/**,
* describe:	����ͬ������ź�ʹ�ܣ�OUT0�����ⲿ����Ĵ����źţ����ͬ������
* type:	    int
* channel:  ����
* value:	1��ʹ��    0���ر�
*/
#define PARAM_TRIGGER_SYNC_OUT                                          "trigger_sync_out"


/**,
* describe: ��������������Ƶ
* type:	    int
* channel:  ����
* value:	1~65535
*/
#define PARAM_ENCODER_DIVISION										   "encoder_division"


/**,
* describe: ����������������ģʽ
* type:	    int
* channel:  ����
* value:	�ο� Confocal_EncoderInputMode_t ����
*/
#define PARAM_ENCODER_INPUT_MODE									   "encoder_input_mode"


/**,
* describe: ����������������ģʽ
* type:	    int
* channel:  ����
* value:	�ο� Confocal_EncoderWorkingMode_t ����
*/
#define PARAM_ENCODER_WORKING_MODE									   "encoder_working_mode"


/**,
* describe: �����������Ƶ����ֵ
* type:	    int
* channel:  ������ͨ��0~3
* value:	0
*/
#define PARAM_ENCODER_CLEAR_COUNT									   "encoder_clear_count"

/**,
* describe: ���ñ�����ͨ��
* type:	    int
* channel:  ����
* value:	0~2
*/
#define PARAM_ENCODER_CHANNEL									       "encoder_channel"



/**,
* describe: ���ô���������ƽ���˲�����
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1~255
*/
#define PARAM_MOVING_AVG_FILTER											  "moving_avg_filter"


/**,
* describe: ���㻬��ƽ���˲�����
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	0
*/
#define PARAM_CLR_MOVING_AVG_FILTER_CNT									   "clr_moving_avg_filter_cnt"

/**,
* describe: ���ô�����������ֵ�˲�����
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1~255
*/
#define PARAM_MOVING_MEDIAN_FILTER										  "moving_median_filter"


/**,
* describe: ���㻬����ֵ�˲�����
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	0
*/
#define PARAM_CLR_MOVING_MEDIAN_FILTER_CNT								  "clr_moving_median_filter_cnt"


/**,
* describe: �������˲�
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1:ʹ��   0:�ر�
*/
#define PARAM_KALMAN_FILTER_EN              							  "kalman_filter_en"

/**,
* describe: �������˲�ϵ��K
* type:	    float
* channel:  ָ������ͷͨ��0~3
* value:	�������˲�ϵ��K
*/
#define PARAM_KALMAN_FILTER_K              								  "kalman_filter_k"


/**,
* describe: �������˲���ֵ
* type:	    float
* channel:  ָ������ͷͨ��0~3
* value:	�������˲���ֵ
*/
#define PARAM_KALMAN_FILTER_THRE             							  "kalman_filter_thre"


/**,
* describe: �������˲�������
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	�������˲�������
*/
#define PARAM_KALMAN_FILTER_NUM             							  "kalman_filter_num"


/**,
* describe: ��Ч�����˲�
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1:ʹ��   0:�ر�
*/
#define PARAM_ERROR_FILTER_EN             							      "error_filter_en"


/**,
* describe: ��Ч�����˲�����
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	��Ч�����˲�����
*/
#define PARAM_ERROR_FILTER_CNT            							      "error_filter_cnt"


/**,
* describe: ѡ��������źţ��û�����ѡ������źš�Զ���źš�ǿ����ǿ���ź�
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	�ο� Confocal_SignalSelect_t
*/
#define PARAM_SELECT_SIGNAL											       "select_signal"


/**,
* describe: ѡ��������źţ��û�����ѡ������źš�Զ���źš�ǿ����ǿ����m
/**,
* describe: ��ȡ��Ӧͨ�����źű��Ͷ�
* type:	    float
* channel:  ָ������ͷͨ��0~3
* value:	�����źű��Ͷ�(0~100)%
*/
#define PARAM_SIGNAL_SATURATION											    "signal_saturation"


/**,
* describe: ����������
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	����������ֵ(0~9)
*/
#define PARAM_MAIN_SIGNAL_INDEX										       "main_signal_index"


/**,
* describe: ���õ���ͨ���Ĳ���ģʽ���������ò��ģʽ������ģʽ
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	�ο� Confocal_MeasureMode_t
*/
#define PARAM_CHANNEL_MEASURE_MODE										   "channel_measure_mode"


/**,
* describe: ���õ�������ͷ�����ģʽ�������⵽�������źž���ֵ
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1:ʹ��   0:�ر�
*/
#define PARAM_CHANNEL_MULT_DIS									 	       "channel_mult_dis"

/**,
* describe: ���õ�������ͷABS����ģʽ��������Բ���ֵ������ZEROӰ��
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1:ʹ��   0:�ر�
*/
#define PARAM_CHANNEL_ABS_MODE										 	    "channel_abs_mode"


/**,
* describe: ���õ�������ͷ����ֵȡ��
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1:ʹ��   0:�ر�
*/
#define PARAM_CHANNEL_REVERSE_VALUE										 	"channel_revere_value"



/**,
* describe: ����ͨ��ƫ�ã���λmm
* type:	    float
* channel:  ָ������ͷͨ��0~3
* value:	ͨ��ƫ��
*/
#define PARAM_CHANNEL_OFFSET												 "channel_offset"


/**,
* describe: �����źż����ֵ��Ϊ����������׼��ı���,Ĭ��ֵΪ3
* type:	    float
* channel:  ָ������ͷͨ��0~3
* value:	0.1~100.0
*/
#define PARAM_SIGNAL_DETECT_THRE										 	 "signal_detect_thre"



/**,
* describe: �����źż���ı���������Ϊ1���ź����ֵ����Сֵ�����е���м���;0.5��ȡ�ź����ֵ����50%��������м���
* type:	    float
* channel:  ָ������ͷͨ��0~3
* value:	0.1~1.0
*/
#define PARAM_SIGNAL_CAL_RATIO    										 	 "signal_cal_ratio"


/**,
* describe: ����Zero�����ֵ
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	0x01:���浱ǰZero���ֵ
*/
#define PARAM_SAVE_ZERO_POSITION    										 	 "save_zero_position"


/**,
* describe: ���Zero�����ֵ
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	0x01:�����ǰZero���ֵ
*/
#define PARAM_CLEAR_ZERO_POSITION    										 	 "clear_zero_position"


/**,
* describe: �Զ��źż��
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:    1:ʹ��   0:�ر�
*/
#define PARAM_AUTO_SIGNAL_DETECT                                                 "auto_signal_detect"


/**,
* describe: �����ż��������(1~6)������ģʽ��Զ��ģʽ�������ģʽ����Ч����������Խ�ߣ�Խ���׼�⵽΢С���ź�
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1~6
*/
#define PARAM_SIGNAL_DETECT_SENSITIVITY                                          "signal_detect_sensitivity"


/**,
* describe: ���ü����źŸ����������⵽���źŸ����������û��趨�ĸ������������Чֵ
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1~10
*/
#define PARAM_SIGNAL_DETECT_NUM     										 "signal_detect_num"



/**,
* describe: �����ź���С�������źŵ���С�ڸ���ֵ�������Чֵ
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1~255
*/
#define PARAM_SIGNAL_MINI_POINTS     										  "signal_mini_points"


/**,
* describe: �����ź�ƽ�����ȣ�Ĭ��ֵΪ8��ƽ������ȥ��һЩ�ź���ë��
* type:	    int
* channel:  ����
* value:	1~255
*/ 
#define PARAM_SIGNAL_SMOOTH										               "signal_smooth"



/**,
* describe: ���źŽ������򣨽��˵�Զ�ˡ�Զ�˵����ˡ����������ң�
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	�ο� Confocal_SignalSort_t
*/
#define PARAM_SIGNAL_SORT  									                   "signal_sort"



/**,
* describe: �������SC_ResultDataTypeDef_t��signal�ֶη����ź����ݵĵ�ַ�����ڵ���
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1��ʹ��   0���ر�
*/
#define PARAM_SIGNAL_DATA_OUTPUT 									            "signal_data_output"



/**,
* describe: �����ڲ������С��Ĭ����X64ϵͳ�¿��Ի���100K����������/X86ϵͳ�¿��Ի���8K����������;�û��ɸ���ʹ�ó������е�����,�û���ͨ��CF_ExportCacheData����Cache����
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1~10000000
*/
#define PARAM_CACHE_SIZE														 "cache_size"


/**,
* describe:��ȡ��ǰ�����ڵ����ݸ���,�û���ͨ��CF_ExportCacheData����Cache����
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	���ص�ǰ��������ݸ���
*/
#define PARAM_CACHE_DATA_CNT													  "cache_data_cnt"


/**,
* describe: ���Cache�ڲ�������
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	0
*/
#define PARAM_CACHE_CLEAR													     "cache_clear"


/**,
* describe: ����Cache������ֵ���������ﵽ��ֵ��ͨ���ص�����֪ͨ�û�,�û���ͨ��CF_ExportCacheData����Cache����
* type:	    int
* channel:  ָ������ͷͨ��0~3
* value:	1~10000000
*/
#define PARAM_CACHE_THRE  													      "cache_thre"

/**,
* describe: ����ģ�������
* type:	    int
* channel:  ����
* value:	1��ʹ��		0���ر�
*/
#define PARAM_ANALOG_VOL_OUTPUT  												   "analog_vol_output"



/**,
* describe: ����RSXXX���Э��:��ѡ��RS485\RS232\RS422���
* type:	    int
* channel:  ����
* value:	�ο� COMM_Protocol_Enum_t
*/
#define PARAM_RSXXX_PROTOCOL 												       "rsxxx_protocol"


/**,
* describe: ����RSXXX������
* type:	    int
* channel:  ����
* value:	�ο� COMM_BaudRate_Enum_t
*/
#define PARAM_RSXXX_BAUDRATE												       "rsxxx_baudrate"



/**,
* describe: ����RSXXX��żУ��
* type:	    int
* channel:  ����
* value:	�ο� COMM_Parity_Enum_t
*/	
#define PARAM_RSXXX_PARITY															"rsxxx_parity"




/**,
* describe: ����RSXXX���ݽ���������Hexģʽ��ASCIIģʽ
* type:	    int
* channel:  ����
* value:	�ο� COMM_Data_Format_Enum_t
*/
#define PARAM_RSXXX_DATA_FORMAT														"rsxxx_data_format"


/**,
* describe: ����RSXXXЭ��ѡ��ʽ���ⲿӲ��IOѡ������ѡ��,Ĭ��ͨ��SDKѡ��
* type:	    int
* channel:  ����
* value:	�ο� COMM_Protocol_Control_Enum_t
*/
#define PARAM_RSXXX_SELECT															"rsxxx_select"




/**,
* describe: ����˫ͷ���ģʽ
* type:	    int
* channel:  ����
* value:	1��ʹ��    0���ر�
*/
#define PARAM_DOUBLE_CHANNEL_MODE													 "double_channel_mode"


/**,
* describe: ˫ͷ���ģʽ�£����ý����Դ�л������ڲ�����͸������
* type:	    int
* channel:  ����
* value:	1��ʹ��    0���ر�
*/
#define PARAM_DOUBLE_CHANNEL_AL_MODE												  "double_channel_al_mode"


/**,
* describe: ˫ͷ���ģʽ�£���ת��������ͷ�Ĳ���ֵ
* type:	    int
* channel:  0~1  0:��Ӧ����ͷ0/1ͨ����ɵ�˫ͷ���    1:��Ӧ����ͷ2/3ͨ����ɵ�˫ͷ���
* value:	1��ʹ��    0���ر�
*/
#define PARAM_DOUBLE_CHANNEL_REVERSE_DIS											   "double_channel_reverse_dis"


/**,
* describe: ˫ͷ���ģʽ�£����ñ궨ģʽ��
*		    �ڱ궨ģʽ���û���Ҫʹ�ñ�׼��ȵĲ��������˫ͷ����������б궨
*			�ڷǱ궨ģʽ�£��û�����Ҫʹ�ñ�׼���������б궨����ʹ��ǰ��Ҫ����������ͷ�����׼��Ȼ�����CF_Zero���й���
* type:	    int
* channel:  0~1  0:��Ӧ����ͷ0/1ͨ����ɵ�˫ͷ���    1:��Ӧ����ͷ2/3ͨ����ɵ�˫ͷ���
* value:	1��ʹ��    0���ر�
*/
#define PARAM_DOUBLE_CHANNEL_CAL_MODE											       "double_channel_cal_mode"


/**,
* describe: ˫ͷ���궨ϵ��a
* type:	    double
* channel:  0~1  0:��Ӧ����ͷ0/1ͨ����ɵ�˫ͷ���    1:��Ӧ����ͷ2/3ͨ����ɵ�˫ͷ���
* value:    �궨ϵ��a
*/
#define PARAM_DOUBLE_CHANNEL_CAL_COEF_A											        "double_channel_cal_coef_a"


/**,
* describe: ˫ͷ���궨ϵ��b
* type:	    double
* channel:  0~1  0:��Ӧ����ͷ0/1ͨ����ɵ�˫ͷ���    1:��Ӧ����ͷ2/3ͨ����ɵ�˫ͷ���
* value:    �궨ϵ��b
*/
#define PARAM_DOUBLE_CHANNEL_CAL_COEF_B											        "double_channel_cal_coef_b"


/**,
* describe: ˫ͷ���궨ϵ��c
* type:	    double
* channel:  0~1  0:��Ӧ����ͷ0/1ͨ����ɵ�˫ͷ���    1:��Ӧ����ͷ2/3ͨ����ɵ�˫ͷ���
* value:    �궨ϵ��c
*/
#define PARAM_DOUBLE_CHANNEL_CAL_COEF_C											        "double_channel_cal_coef_c"


/**,
* describe: ʹ��trigger pass ���Թ���,�ɹ�ʹ�ܺ󷵻صĽ������ bTriggerPass��������ָʾ�Ƿ�trigger pass
* type:	    int
* channel:  ����
* value:	1��ʹ��    0���ر�
*/
#define PARAM_TRIGGER_PASS_DEBUG    											        "trigger_pass_debug"

/**,
* describe:	���trigger pass ��־λ
* type:	    int
* channel:  ����
* value:	0
*/
#define PARAM_CLEAR_TRIGGER_PASS_FLAG    											      "clear_trigger_pass_flag"

/**,
* describe:	�Ƿ�����������
* type:	    int
* channel:  ����
* value:	1��ʹ��    0���ر�
*/
#define PARAM_SUB_BACKGROUND_LIGHT                                                         "sub_background_light"



/**,
* describe:	�����ⲿIO��IN0�Ĺ���
* type:	    int
* channel:  ��IO�ڹ����Ĵ�����ͨ��
* value:	Confocal_InputPortFunc_t
*/
#define PARAM_SYNC_IN0_FUNC                                                                  "sync_in0_func"


/**,
* describe:	�����ⲿIO��IN1�Ĺ���
* type:	    int
* channel:  ��IO�ڹ����Ĵ�����ͨ��
* value:	Confocal_InputPortFunc_t
*/
#define PARAM_SYNC_IN1_FUNC                                                                   "sync_in1_func"



/**,
* describe:	�ⲿ���봥������ʱ��
* type:	    int
* channel:  ����
* value:	����ʱ�䣬��λms
*/
#define PARAM_EX_TRIGGER_DEBOUNCE                                                              "ex_trigger_debounce"


#endif
