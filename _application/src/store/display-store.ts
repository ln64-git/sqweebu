import {create} from "zustand";

interface DisplayStore {
  sidebar: boolean;
  scrollBottom: boolean;
  viewWidth: number;
  viewHeight: number;
  toggleSidebar: () => void;
  setScrollBottom: (shouldScroll: boolean) => void;
  setViewWidth: (width: number) => void;
  setViewHeight: (height: number) => void;
}

export const useDisplayStore = create<DisplayStore>((set) => ({
  sidebar: false,
  scrollBottom: false,
  viewWidth: window.innerWidth,
  viewHeight: window.innerHeight,
  toggleSidebar: () => set((state) => ({ sidebar: !state.sidebar })),
  setScrollBottom: (shouldScroll) => set(() => ({ scrollBottom: shouldScroll })),
  setViewWidth: (width) => set(() => ({ viewWidth: width })),
  setViewHeight: (height) => set(() => ({ viewHeight: height })),
}));
