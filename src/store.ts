import { create } from "zustand";

interface Nexus {
  isMobile: boolean;
  sidebar: boolean;
  darkMode: boolean;
  setMobile: (isMobile: boolean) => void;
  setSidebar: (sidebar: boolean) => void;
  setDarkMode: (sidebar: boolean) => void;
}

const useNexus = create<Nexus>()((set) => ({
  isMobile: true,
  sidebar: false,
  darkMode: true,
  setMobile: (isMobile) => set((state) => ({ ...state, isMobile: isMobile })),
  setSidebar: (sidebar) => set((state) => ({ ...state, sidebar: sidebar })),
  setDarkMode: (darkMode) => set((state) => ({ ...state, darkMode: darkMode })),
}));

export default useNexus;
