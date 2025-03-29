import { create } from "zustand";
import { invoke } from '@tauri-apps/api/core';

interface SoftwareSettings {
  isDevMode: boolean;
  selectedArtifactType: 'EH09'|'EH12'|'EK30'|'EK40'|'EY28'|'TEST';
  toggleDevMode: () => Promise<void>;
  setSelectedArtifactType:(atype:'EH09'|'EH12'|'EY28'|'EK30'|'EK40'|'EY28'|'TEST') => void;
}

export const useSoftwareSettingsStore = create<SoftwareSettings>((set, get) => ({
  isDevMode: false, // 默认关闭
  selectedArtifactType: 'EH09',
  setSelectedArtifactType: (log: 'EH09'|'EH12'|'EY28'|'EK30'|'EK40'|'EY28'|'TEST') => set(() => ({ selectedArtifactType: log })),
  toggleDevMode: async () => {
    const currentState = get().isDevMode;
    try {
      if (!currentState) {
        await invoke("toggle_hardware_dev_on");
        console.log("机器人进入 Dev 模式");
      } else {
        await invoke("toggle_hardware_dev_off");
        console.log("机器人退出 Dev 模式");
      }
      set({ isDevMode: !currentState }); // 更新状态
    } catch (error) {
      console.error("切换 Dev 模式失败:", error);
    }
  },
}));
