import { create } from "zustand";
import { defaultLightTheme, defaultDarkTheme, Theme } from "./config/themes";

interface Nexus {
  isMobile: boolean;
  sidebar: boolean;
  darkMode: boolean;
  lightTheme: Theme;
  darkTheme: Theme;
  setMobile: (isMobile: boolean) => void;
  setSidebar: (sidebar: boolean) => void;
  setDarkMode: (sidebar: boolean) => void;
  setLightTheme: (theme: Partial<Theme>) => void;
  setDarkTheme: (theme: Partial<Theme>) => void;
}

const useNexus = create<Nexus>((set) => ({
  isMobile: true,
  sidebar: false,
  darkMode: true,
  lightTheme: defaultLightTheme,
  darkTheme: defaultDarkTheme,
  setMobile: (isMobile) => set((state) => ({ ...state, isMobile })),
  setSidebar: (sidebar) => set((state) => ({ ...state, sidebar })),
  setDarkMode: (darkMode) => set((state) => ({ ...state, darkMode: darkMode })),
  setLightTheme: (theme) =>
    set((state) => ({
      ...state,
      lightTheme: { ...state.lightTheme, ...theme },
    })),
  setDarkTheme: (theme) =>
    set((state) => ({ ...state, darkTheme: { ...state.darkTheme, ...theme } })),
}));

export default useNexus;
