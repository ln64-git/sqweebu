import { Theme, defaultDarkTheme, defaultLightTheme } from "@/config/themes";
import {create} from "zustand";

interface ThemeStore {
  darkMode: boolean;
  lightTheme: Theme;
  darkTheme: Theme;
  toggleDarkMode: () => void;
  setLightTheme: (theme: Partial<Theme>) => void;
  setDarkTheme: (theme: Partial<Theme>) => void;
}

export const useThemeStore = create<ThemeStore>((set) => ({
  darkMode: true, 
  lightTheme: defaultLightTheme,
  darkTheme: defaultDarkTheme,
  toggleDarkMode: () => set((state) => ({ darkMode: !state.darkMode })),
  setLightTheme: (theme) => set((state) => ({ lightTheme: { ...state.lightTheme, ...theme } })),
  setDarkTheme: (theme) => set((state) => ({ darkTheme: { ...state.darkTheme, ...theme } })),
}));
