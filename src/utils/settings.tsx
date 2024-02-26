import useUserSettingsStore, { UserSettings } from "@/user-settings-store";
import { invoke } from "@tauri-apps/api";

export const getUserSettings = async () => {
  try {
    const response = await invoke<string>("get_user_settings_as_json");
    const settingsObject = JSON.parse(response);
    useUserSettingsStore.setState({ userSettings: settingsObject });
  } catch (error) {
    console.error("Failed to fetch backend settings:", error);
  }
};

export const setUserSettings = async (settings: UserSettings) => {
  try {
    const settingsString = JSON.stringify(settings);
    await invoke("set_user_settings_from_json", {
      settings_json: settingsString,
    });
    useUserSettingsStore.setState({ userSettings: settings });
  } catch (error) {
    console.error("Failed to set backend settings:", error);
  }
};
