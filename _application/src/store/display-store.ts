import {create} from "zustand";

interface DisplayStore {
  sidebar: boolean;
  viewWidth: number;
  viewHeight: number;
  toggleSidebar: () => void;
  setViewWidth: (width: number) => void;
  setViewHeight: (height: number) => void;
}

export const useDisplayStore = create<DisplayStore>((set) => ({
  sidebar: false,
  viewWidth: window.innerWidth,
  viewHeight: window.innerHeight,
  toggleSidebar: () => set((state) => ({ sidebar: !state.sidebar })),
  setViewWidth: (width) => set(() => ({ viewWidth: width })),
  setViewHeight: (height) => set(() => ({ viewHeight: height })),
}));
