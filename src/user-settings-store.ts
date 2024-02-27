import { create } from "zustand";
import { defaultLightTheme, defaultDarkTheme, Theme } from "./config/themes";

interface UserSettingsInterface {
  gptService: string;
  gptModel: string;
  speechService: string;
  speechLocale: string;
  speechVoice: string;
  currentUserId: string;
  currentUserTheme: Theme;
  currentUserThemeMode: string;
  setGptService: (service: string) => void;
  setGptModel: (model: string) => void;
  setSpeechService: (service: string) => void;
  setSpeechLocale: (locale: string) => void;
  setSpeechVoice: (voice: string) => void;
  setCurrentUserId: (id: string) => void;
  setCurrentUserTheme: (theme: Partial<Theme>) => void;
  setCurrentUserThemeMode: (mode: string) => void;
}

const userSettingsStore = create<UserSettingsInterface>((set) => ({
  gptService: "",
  gptModel: "",
  speechService: "",
  speechLocale: "",
  speechVoice: "",
  currentUserId: "",
  currentUserTheme: defaultLightTheme,
  currentUserThemeMode: "",
  setGptService: (service) =>
    set((state) => ({ ...state, gptService: service })),
  setGptModel: (model) => set((state) => ({ ...state, gptModel: model })),
  setSpeechService: (service) =>
    set((state) => ({ ...state, speechService: service })),
  setSpeechLocale: (locale) =>
    set((state) => ({ ...state, speechLocale: locale })),
  setSpeechVoice: (voice) => set((state) => ({ ...state, speechVoice: voice })),
  setCurrentUserId: (id) => set((state) => ({ ...state, currentUserId: id })),
  setCurrentUserTheme: (theme) =>
    set((state) => ({
      ...state,
      currentUserTheme: { ...state.currentUserTheme, ...theme },
    })),
  setCurrentUserThemeMode: (mode) =>
    set((state) => ({ ...state, currentUserThemeMode: mode })),
}));

export default userSettingsStore;
