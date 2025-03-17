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

interface DashboardState {
    artifactType: string;
    statics: string;
    artifact: string;
    logs: string;
    images_1: Blob[];
    images_2: Blob[];
    info_1: string;
    info_2: string;
    logComponentValue: LogShow[]; // 子组件状态示例
    resultComponentValue: SurfaceData[];
    // 方法
    setLogs: (log: string) => void;
    addLogComponentValueEntry: (log: LogShow) => void;
    addImage_1: (image: Blob) => void;
    addImage_2: (image: Blob) => void;
    clearImage_1: () => void;
    clearImage_2: () => void;
    updateResultComponent: (
      surface: string, // 要更新的 surface 名称
      newStatus?: "OK" | "NG" | "NULL", // 更新的状态
      newHoles?: (boolean | null)[] // 更新的 holes 数组
    ) => void;
  }

export const useDashboardStore = create<DashboardState>((set) => ({
    artifactType:"EK40",
    statics:"+120",
    artifact:"100",
    logs:"",
    images_1: [],
    images_2: [],
    info_1: "",
    info_2: "",
    logComponentValue: [{ sender: "System", level: "info", info: "程序启动." }],
    resultComponentValue: [
        { surface: "A", status: "OK", holes: [true, true, true, false, true] },
        { surface: "B", status: "NG", holes: [false, false, false, false, false] },
        { surface: "C", status: "OK", holes: [true, true, true, true, true] },
        { surface: "D", status: "OK", holes: [true, true, true, false, true, true, false, true, true] },
        { surface: "E", status: "NULL", holes: [null, null, null, null, null, true, false, true, true, true, false, true, true, false] },
        { surface: "F", status: "OK", holes: [false, true, true, false] }
      ],
    setLogs: (log) => set((state) => ({ logs: state.logs + `\n${log}` })),
    addLogComponentValueEntry: (log) =>
    set((state) => ({ logComponentValue: [...state.logComponentValue, log] })),
    addImage_1: (image) => set((state) => ({ images_1: [...state.images_1, image] })),
    addImage_2: (image) => set((state) => ({ images_2: [...state.images_2, image] })),
    clearImage_1: () => set(() => ({ images_1: [] })),
    clearImage_2: () => set(() => ({ images_2: [] })),
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