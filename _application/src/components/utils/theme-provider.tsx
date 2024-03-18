import { Theme, defaultDarkTheme, defaultLightTheme } from "@/config/themes";
import React, {
  createContext,
  useContext,
  ReactNode,
  useState,
  useEffect,
} from "react";

interface ThemeContextType {
  theme: Theme;
  toggleTheme: () => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export const ThemeProvider = ({ children }: { children: ReactNode }) => {
  const [theme, setTheme] = useState<Theme>(defaultLightTheme);
  const [isDarkMode, setIsDarkMode] = useState<boolean>(true);

  // Example toggle function (adjust as needed)
  const toggleTheme = () => {
    setIsDarkMode(!isDarkMode);
  };

  // Switch themes based on isDarkMode
  useEffect(() => {
    setTheme(isDarkMode ? defaultDarkTheme : defaultLightTheme);
  }, [isDarkMode]);
  const backgroundColor = theme.background;

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      <div
        style={{ backgroundColor: backgroundColor }}
        className="h-screen flex flex-col items-center justify-center"
      >
        {children}
      </div>
    </ThemeContext.Provider>
  );
};

// Custom hook to use the theme context
export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error("useTheme must be used within a ThemeProvider");
  }
  return context;
};
