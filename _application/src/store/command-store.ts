import {create} from "zustand";

interface CommandStore {
  command: string;
  flashCommand: (command: string) => void;
}

export const useCommandStore = create<CommandStore>((set) => ({
  command: "",
  flashCommand: (command) => {
    set(() => ({ command }));
    setTimeout(() => set(() => ({ command: "" })), 1000); 
  },
}));
