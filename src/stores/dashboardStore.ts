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
    logComponentValue: LogShow[]; // 子组件状态示例
    resultComponentValue: SurfaceData[];
    systemstate: SystemState;
    // 方法
    setIsRunning:(state:boolean)=>void;
    setArtifactType:(atype: string)=>void;
    // setStatics:(statics: string)=>void;
    // setArtifact:(result: string)=>void;
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
    updateResultComponent: (
      surface: string, // 要更新的 surface 名称
      newStatus?: "OK" | "NG" | "NULL", // 更新的状态
      newHoles?: (boolean | null)[] // 更新的 holes 数组
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
  setArtifactType:(atype:string) => set({artifactType:atype}),
  setIsRunning: (state: boolean) => set({ isRunning: state }),
  setLogs: (log) => set((state) => ({ logs: state.logs + `\n${log}` })),
  addLogComponentValueEntry: (log) =>
  set((state) => ({ logComponentValue: [...state.logComponentValue, log] })),
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
  setInfo_2:   (info:string) => set((state) => {
    if (state.info_2 !== info) {
      return { info_2: info };
    }
    return state;}),
  clearInfo_1: () => set(() => ({ image_1: null })),
  clearInfo_2: () => set(() => ({ image_2: null })),

  updateResultComponent: (surface, newStatus, newHoles) =>
    set((state) => ({
      resultComponentValue: state.resultComponentValue.map((item) =>
        item.surface === surface
          ? {
              ...item,
              status: newStatus !== undefined ? newStatus : item.status, // 如果传入了新的状态，更新状态，否则保留旧状态
              holes: newHoles !== undefined ? newHoles : item.holes, // 如果传入了新的 holes，更新 holes，否则保留旧 holes
            }
          : item // 其他 item 保持不变
      ),
    })),
  }));