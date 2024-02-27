import { create } from "zustand";
import { defaultDarkTheme, Theme } from "./config/themes";

interface Nexus {
  sidebar: boolean;
  darkMode: boolean;
  command: string;
  viewWidth: number;
  viewHeight: number;
  setSidebar: (sidebar: boolean) => void;
  setDarkMode: (darkMode: boolean) => void;
  flashCommand: (command: string) => void;
  setViewWidth: (viewWidth: number) => void;
  setViewHeight: (viewHeight: number) => void;
}

const useInterfaceStore = create<Nexus>((set) => ({
  sidebar: false,
  darkMode: true,
  command: "",
  viewWidth: 0,
  viewHeight: 0,
  setSidebar: (sidebar) => set((state) => ({ ...state, sidebar })),
  setDarkMode: (darkMode) => set((state) => ({ ...state, darkMode })),
  flashCommand: (command) => {
    set((state) => ({ ...state, command }));
    setTimeout(() => {
      set((state) => ({ ...state, command: "" }));
    }, 1000); // Reset state after 1 second
  },
  setViewWidth: (viewWidth) => set((state) => ({ ...state, viewWidth })),
  setViewHeight: (viewHeight) => set((state) => ({ ...state, viewHeight })),
}));

export default useInterfaceStore;
