import useUserSettingsStore, { UserSettings } from "@/user-settings-store";
import { invoke } from "@tauri-apps/api";

export const readUserSettings = async () => {
  try {
    const response = await invoke<string>("get_user_settings_as_json");
    if (response) {
      const parsedSettings = JSON.parse(response);
      const camelCaseSettings = convertSnakeCaseToCamelCase(parsedSettings);
      console.log("Received settings from backend:", camelCaseSettings);
      useUserSettingsStore.setState(camelCaseSettings);
    } else {
      console.log("Response from backend is empty. Loading default settings.");
    }
  } catch (error) {
    console.error("Failed to fetch backend settings:", error);
  }
};

export const saveUserSettings = async (settings: UserSettings) => {
  try {
    const snakeCaseSettings = convertCamelCaseToSnakeCase(settings);
    const settingsString = JSON.stringify(snakeCaseSettings);
    console.log("Updating backend settings:", snakeCaseSettings);
    await invoke("save_user_settings_from_json", {
      settingsJson: settingsString,
    });
  } catch (error) {
    console.error("Failed to update backend settings:", error);
  }
};

const convertSnakeCaseToCamelCase = (obj: any): any => {
  if (typeof obj !== "object" || obj === null) {
    return obj;
  }
  if (Array.isArray(obj)) {
    return obj.map(convertSnakeCaseToCamelCase);
  }
  return Object.keys(obj).reduce((acc: any, key: string) => {
    const camelCaseKey = key.replace(/_([a-z])/g, (match, group) =>
      group.toUpperCase()
    );
    const value = obj[key];
    acc[camelCaseKey] = convertSnakeCaseToCamelCase(value);
    return acc;
  }, {});
};

const convertCamelCaseToSnakeCase = (obj: any): any => {
  if (typeof obj !== "object" || obj === null) {
    return obj;
  }
  if (Array.isArray(obj)) {
    return obj.map(convertCamelCaseToSnakeCase);
  }
  return Object.keys(obj).reduce((acc: any, key: string) => {
    const snakeCaseKey = key.replace(
      /[A-Z]/g,
      (match) => `_${match.toLowerCase()}`
    );
    const value = obj[key];
    acc[snakeCaseKey] = convertCamelCaseToSnakeCase(value);
    return acc;
  }, {});
};
