import { create } from "zustand";

interface Nexus {
  isMobile: boolean;
  sidebar: boolean;
  setMobile: (isMobile: boolean) => void;
  setSidebar: (sidebar: boolean) => void;
}

const useNexus = create<Nexus>()((set) => ({
  isMobile: true,
  sidebar: false,
  setMobile: (isMobile) => set((state) => ({ ...state, isMobile: isMobile })),
  setSidebar: (sidebar) => set((state) => ({ ...state, sidebar: sidebar })),
}));

export default useNexus;
