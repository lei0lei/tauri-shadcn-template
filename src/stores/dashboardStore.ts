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
    info_1: string;
    info_2: string;
    logComponentValue: LogShow[]; // 子组件状态示例
    resultComponentValue: SurfaceData[];
    systemstate: SystemState;
    // 方法
    setIsRunning:(state:boolean)=>void;
    // setArtifactType:(atype: string)=>void;
    // setStatics:(statics: string)=>void;
    // setArtifact:(result: string)=>void;
    // setSystemstate:(state: SystemState)=>void;

    setLogs: (log: string) => void;
    addLogComponentValueEntry: (log: LogShow) => void;
    addImage_1: (image: string) => void;
    addImage_2: (image: string) => void;
    clearImage_1: () => void;
    clearImage_2: () => void;
    updateResultComponent: (
      surface: string, // 要更新的 surface 名称
      newStatus?: "OK" | "NG" | "NULL", // 更新的状态
      newHoles?: (boolean | null)[] // 更新的 holes 数组
    ) => void;
  }

export const useDashboardStore = create<DashboardState>((set) => ({
  isRunning:false,  
  artifactType:"-",
  statics:"-",
  artifact:"-",
  logs:"",
  image_1: null,
  image_2: null,
  info_1: "-",
  info_2: "-",
  systemstate: {camera_connected:null,plc_connected:null,robot_connected:null,sensor_connected:null,algo:null,hardware:null},

  logComponentValue: [{ sender: "System", level: "info", info: "程序启动." }],
  resultComponentValue: [
      { surface: "A", status: "OK", holes: [true, true, true, false, true] },
      { surface: "B", status: "NG", holes: [false, false, false, false, false] },
      { surface: "C", status: "OK", holes: [true, true, true, true, true] },
      { surface: "D", status: "OK", holes: [true, true, true, false, true, true, false, true, true] },
      { surface: "E", status: "NULL", holes: [null, null, null, null, null, true, false, true, true, true, false, true, true, false] },
      { surface: "F", status: "OK", holes: [false, true, true, false] }
    ],
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