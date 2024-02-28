import { create } from "zustand";
import { defaultDarkTheme, Theme } from "./config/themes";

export interface UserSettings {
  gptMethod: string;
  gptModel: string;
  speechService: string;
  speechLocale: string;
  speechVoice: string;
  currentUserId: string;
  currentUserDarkMode: boolean;
  setGptMethod: (service: string) => void;
  setGptModel: (model: string) => void;
  setSpeechService: (service: string) => void;
  setSpeechLocale: (locale: string) => void;
  setSpeechVoice: (voice: string) => void;
  setCurrentUserId: (id: string) => void;
  setCurrentUserDarkMode: (mode: boolean) => void;
}

import { readUserSettings } from "@/utils/settings"; // Import the readUserSettings function

const userSettingsStore = create<UserSettings>((set) => {
  // Initialize state with default values
  const initialState: UserSettings = {
    gptMethod: "",
    gptModel: "",
    speechService: "",
    speechLocale: "",
    speechVoice: "",
    currentUserId: "",
    currentUserDarkMode: true,
    setGptMethod: (service) =>
      set((state) => ({ ...state, gptMethod: service })),
    setGptModel: (model) => set((state) => ({ ...state, gptModel: model })),
    setSpeechService: (service) =>
      set((state) => ({ ...state, speechService: service })),
    setSpeechLocale: (locale) =>
      set((state) => ({ ...state, speechLocale: locale })),
    setSpeechVoice: (voice) =>
      set((state) => ({ ...state, speechVoice: voice })),
    setCurrentUserId: (id) => set((state) => ({ ...state, currentUserId: id })),
    setCurrentUserDarkMode: (mode) =>
      set((state) => ({ ...state, currentUserDarkMode: mode })),
  };

  // Fetch user settings and update state
  return initialState; // Return initial state immediately
});

export default userSettingsStore;
