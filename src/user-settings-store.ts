import { create } from "zustand";

export interface UserSettings {
  gpt_service: string;
  gpt_model: string;
  speech_service: string;
  speech_local: string;
  speech_voice: string;
  current_user_id: string;
  current_user_theme: string;
  current_user_theme_mode: string;
}

interface UserSettingsState {
  userSettings: UserSettings;
  setUserSettings: (settings: UserSettings) => void;
}

const useUserSettingsStore = create<UserSettingsState>((set) => ({
  userSettings: {
    gpt_service: "",
    gpt_model: "",
    speech_service: "",
    speech_local: "",
    speech_voice: "",
    current_user_id: "",
    current_user_theme: "",
    current_user_theme_mode: "",
  },
  setUserSettings: (settings) => set({ userSettings: settings }),
}));

export default useUserSettingsStore;
