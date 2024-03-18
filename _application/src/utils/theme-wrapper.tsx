// ThemeWrapper.tsx
import { ReactNode } from "react";
import { useThemeColor } from "@/config/themes";

interface ThemeWrapperProps {
  children: ReactNode;
}

const ThemeWrapper: React.FC<ThemeWrapperProps> = ({ children }) => {
  const backgroundColor = useThemeColor("background");
  console.log(backgroundColor);
  const textPrimary = useThemeColor("textPrimary");
  console.log(textPrimary);

  return (
    <div
      style={{ backgroundColor, color: textPrimary }}
      className="flex flex-col h-screen"
    >
      {children}
    </div>
  );
};

export default ThemeWrapper;
