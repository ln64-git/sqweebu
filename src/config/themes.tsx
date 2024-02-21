export interface Theme {
  background: string;
  input: string;
  overlay: string;
  accent: string;
}

export const defaultLightTheme = {
  background: "#797974",
  input: "#ffffff",
  overlay: "#ffffff",
  accent: "#ffffff",
};

export const defaultDarkTheme: Theme = {
  background: "bg-zinc-900",
  input: "#000000",
  overlay: "#000000",
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
