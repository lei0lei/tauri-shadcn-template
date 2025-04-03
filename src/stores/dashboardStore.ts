import { create } from "zustand";
interface LogShow {
    sender: string;
    level: string;
    info: string;
  }

  interface SurfaceData {
    surface: string;
    status: "OK" | "NG" | "NULL";
    holes: (boolean | null)[];
  }

  interface SystemState {
    camera_connected: boolean | null;
    plc_connected: boolean | null;
    robot_connected: boolean | null;
    sensor_connected: boolean | null;
    algo: boolean |null;
    hardware: string|null;
  }

interface DashboardState {
    isRunning: boolean;
    artifactType: string;
    statics: string;
    artifact: string;
    logs: string;
    image_1: string|null;
    image_2: string|null;
    info_1: string|null;
    info_2: string|null;
    current_hole: string|null;
    current_face: string|null;
    logComponentValue: LogShow[]; // 子组件状态示例
    resultComponentValue: SurfaceData[];
    systemstate: SystemState;
    // 方法
    setIsRunning:(state:boolean)=>void;
    setArtifactType:(atype: string)=>void;
    // setStatics:(statics: string)=>void;
    setArtifact:(result: string)=>void;
    // setSystemstate:(state: SystemState)=>void;

    setLogs: (log: string) => void;
    addLogComponentValueEntry: (log: LogShow) => void;
    addImage_1: (image: string) => void;
    addImage_2: (image: string) => void;
    setInfo_1:(info: string)=>void;
    setInfo_2:(info: string)=>void;
    clearImage_1: () => void;
    clearImage_2: () => void;
    clearInfo_1: () => void;
    clearInfo_2: () => void;
    setCurrentHole:(hole: string, face:string)=>void;
    updateResultComponent: (
      surface: string,                      // 要更新的 surface 名称
      options?: { 
        newStatus?: "OK" | "NG" | "NULL";   // 更新整个面的状态（可选）
        newHoles?: (boolean | null)[];      // 更新整个面的 holes（可选）
        holeIndex?: number;                 // 更新某个特定孔（可选）
        holeState?: boolean | null;         // 要更新的孔的新状态（可选）
      }
    ) => void;
  }

export const useDashboardStore = create<DashboardState>((set) => ({
  isRunning:false,  
  artifactType:"---",
  statics:"---",
  artifact:"---",
  logs:"",
  image_1: null,
  image_2: null,
  info_1: "---",
  info_2: "---",
  current_hole:"---",
  current_face:"---",
  systemstate: {camera_connected:null,plc_connected:null,robot_connected:null,sensor_connected:null,algo:null,hardware:null},

  logComponentValue: [{ sender: "System", level: "info", info: "程序启动." }],
  resultComponentValue: [
      { surface: "A", status: "NULL", holes: [] },
      { surface: "B", status: "NULL", holes: [] },
      { surface: "C", status: "NULL", holes: [] },
      { surface: "D", status: "NULL", holes: [] },
      { surface: "E", status: "NULL", holes: [] },
      { surface: "F", status: "NULL", holes: [] }
    ],
  setArtifactType: (atype: string) => {
    set({ artifactType: atype });

    // 根据不同型号设置 holes
    const holesPreset: Record<string, SurfaceData[]> = {
      "EH09": [
        { surface: "A", status: "NULL", holes: [null, null] },
        { surface: "B", status: "NULL", holes: [null,null,null,null,null,null,null,null,null,] },
        { surface: "C", status: "NULL", holes: [null, null, null,null,null,] },
        { surface: "D", status: "NULL", holes: [null,null,null,null,null,null,null,null,null,null,null,null,] },
        { surface: "E", status: "NULL", holes: [null,null,null,null,null,null,] },
        { surface: "F", status: "NULL", holes: [null,null,null,null,] }
      ],
      "EH12": [
        { surface: "A", status: "NULL", holes: [false, true] },
        { surface: "B", status: "NULL", holes: [true, false, true] },
        { surface: "C", status: "NULL", holes: [false, false, false] },
        { surface: "D", status: "NULL", holes: [true] },
        { surface: "E", status: "NULL", holes: [] },
        { surface: "F", status: "NULL", holes: [null, true] }
      ],
      "EK30": [ /* 其他型号的配置... */ ],
      "EK40": [ /* 其他型号的配置... */ ],
      "EY28": [ /* 其他型号的配置... */ ],
      "TEST": [
        { surface: "A", status: "NULL", holes: [] },
        { surface: "B", status: "NULL", holes: [] },
        { surface: "C", status: "NULL", holes: [] },
        { surface: "D", status: "NULL", holes: [] },
        { surface: "E", status: "NULL", holes: [] },
        { surface: "F", status: "NULL", holes: [] }
      ],
    };

    set({ resultComponentValue: holesPreset[atype] || [] });
  },
  setIsRunning: (state: boolean) => set({ isRunning: state }),
  setLogs: (log) => set((state) => ({ logs: state.logs + `\n${log}` })),
  addLogComponentValueEntry: (log) =>
  set((state) => ({ logComponentValue: [...state.logComponentValue, log].slice(-100),  })),
  addImage_1: (image: string) => set((state) => {
    // 只有在新图片和当前图片不同的时候才更新
    if (state.image_1 !== image) {
      return { image_1: image };
    }
    return state; // 如果图片没有变化，保持不变
  }),
  addImage_2: (image: string) => set((state) => {
    // 只有在新图片和当前图片不同的时候才更新
    if (state.image_2 !== image) {
      return { image_2: image };
    }
    return state; // 如果图片没有变化，保持不变
  }),
  clearImage_1: () => set(() => ({ image_1: null })),
  clearImage_2: () => set(() => ({ image_2: null })),

  setInfo_1:   (info:string) => set((state) => {
    if (state.info_1 !== info) {
      return { info_1: info };
    }
    return state;}), // 如果图片没有变化，保持不变),
  setArtifact:   (artifact:string) => set((state) => {
    if (state.artifact !== artifact) {
      return { artifact: artifact };
    }
    return state;}), // 如果图片没有变化，保持不变),
  setInfo_2:   (info:string) => set((state) => {
    if (state.info_2 !== info) {
      return { info_2: info };
    }
    return state;}),
  setCurrentHole:   (hole:string, face:string) => set((state) => {
    if (state.current_hole !== hole|| state.current_face !== face ) {
      return { current_hole: hole, current_face: face};
    }
    return state;}), // 如果图片没有变化，保持不变),
  clearInfo_1: () => set(() => ({ image_1: null })),
  clearInfo_2: () => set(() => ({ image_2: null })),

  updateResultComponent: (surface, options) =>
    set((state) => {
      const updatedResult = state.resultComponentValue.map((item) => {
        if (item.surface === surface) {
          let newHoles = [...item.holes];
  
          // 更新特定的 hole
          if (options?.holeIndex !== undefined && options?.holeState !== undefined) {
            const adjustedIndex = options.holeIndex - 1; // 关键点
            if (adjustedIndex >= 0 && adjustedIndex < newHoles.length) {
              newHoles[adjustedIndex] = options.holeState;
            }
          }
  
          // 如果提供了 newHoles，整体更新 holes
          if (options?.newHoles) {
            newHoles = options.newHoles;
          }
  
          // 计算新的 status
          let newStatus: "OK" | "NG" | "NULL" = "NULL";
          if (newHoles.includes(false)) {
            newStatus = "NG";
          } else if (newHoles.length > 0 && newHoles.every(h => h === true)) {
            newStatus = "OK";
          }
  
          // 如果手动指定 newStatus，则覆盖自动计算的状态
          if (options?.newStatus) {
            newStatus = options.newStatus;
          }
  
          return { ...item, holes: newHoles, status: newStatus };
        }
        return item;
      });
  
      return { resultComponentValue: updatedResult };
    }),
  }));