import { create } from "zustand";

interface CommandStore {
  mode: string;
  command: string;
  setMode: (mode: string) => void; 
  flashCommand: (command: string) => void;
}

export const useCommandStore = create<CommandStore>((set) => ({
  mode: "",
  command: "", 
  setMode: (mode) => {
    set(() => ({ mode })); 
  },
  flashCommand: (command) => {
    set(() => ({ command }));
    setTimeout(() => set(() => ({ command: "" })), 1000); 
  },
}));
