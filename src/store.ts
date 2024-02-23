import { create } from "zustand";
import { defaultLightTheme, defaultDarkTheme, Theme } from "./config/themes";

interface Nexus {
  sidebar: boolean;
  darkMode: boolean;
  lightTheme: Theme;
  darkTheme: Theme;
  command: string;
  viewWidth: number;
  viewHeight: number;
  setSidebar: (sidebar: boolean) => void;
  setDarkMode: (sidebar: boolean) => void;
  setLightTheme: (theme: Partial<Theme>) => void;
  setDarkTheme: (theme: Partial<Theme>) => void;
  flashCommand: (command: string) => void;
  setViewWidth: (viewWidth: number) => void;
  setViewHeight: (viewHeight: number) => void;
}

const useNexus = create<Nexus>((set) => ({
  sidebar: false,
  darkMode: true,
  command: "",
  lightTheme: defaultLightTheme,
  darkTheme: defaultDarkTheme,
  viewWidth: 0,
  viewHeight: 0,
  setSidebar: (sidebar) => set((state) => ({ ...state, sidebar })),
  setDarkMode: (darkMode) => set((state) => ({ ...state, darkMode })),
  setLightTheme: (theme) =>
    set((state) => ({
      ...state,
      lightTheme: { ...state.lightTheme, ...theme },
    })),
  setDarkTheme: (theme) =>
    set((state) => ({ ...state, darkTheme: { ...state.darkTheme, ...theme } })),
  flashCommand: (command) => {
    set((state) => ({ ...state, command }));
    setTimeout(() => {
      set((state) => ({ ...state, command: "" }));
    }, 1000); // Reset state after 1 second
  },
  setViewWidth: (viewWidth: number) =>
    set((state) => ({ ...state, viewWidth })),
  setViewHeight: (viewHeight: number) =>
    set((state) => ({ ...state, viewHeight })),
}));

export default useNexus;
