import useNexus from "@/store";
import { useEffect, useState } from "react";

export interface Theme {
  background: string;
  input: string;
  overlay: string;
  accent: string;
}

export function useThemeColor(colorType: keyof Theme): string {
  const darkMode = useNexus((state) => state.darkMode);
  const lightTheme = useNexus((state) => state.lightTheme);
  const darkTheme = useNexus((state) => state.darkTheme);

  const [themeColor, setThemeColor] = useState<string>("");

  useEffect(() => {
    const currentTheme = darkMode ? darkTheme : lightTheme;
    setThemeColor(darkMode ? currentTheme[colorType] : currentTheme[colorType]);
  }, [darkMode, lightTheme, darkTheme, colorType]);

  return themeColor;
}

export const defaultLightTheme = {
  background: "#797974",
  input: "#ffffff",
  overlay: "#ffffff",
  accent: "#ffffff",
};

export const defaultDarkTheme: Theme = {
  background: "#18181b",
  input: "#000000",
  overlay: "#09090b",
  accent: "#000000",
};

export const ubuntuTheme: Theme = {
  background: "#dd4814",
  input: "#dd4814",
  overlay: "#dd4814",
  accent: "#dd4814",
};

export const draculaTheme: Theme = {
  background: "#282a36",
  input: "#282a36",
  overlay: "#282a36",
  accent: "#282a36",
};
