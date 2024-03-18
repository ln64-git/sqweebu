"use client";
import React, { useState } from "react";
import light from "../../../public/sidebar/light.svg";
import dark from "../../../public/sidebar/dark.svg";
import { defaultLightTheme } from "@/config/themes";
import { useTheme } from "../utils/theme-provider";
import HoverableIcon from "../utils/hoverable-icon";

export default function LightSwitch() {
  const { theme, toggleTheme } = useTheme(); // Destructure to get toggleTheme
  const [isLightOn, setIsLightOn] = useState(theme === defaultLightTheme); // Use theme to determine if it's light or dark

  const toggleLight = () => {
    toggleTheme(); // This will toggle the theme between light and dark
    setIsLightOn((prev) => !prev); // Toggle the local state as well
  };
  return (
    <div className="flex flex-col items-center gap-2 pt-3 pb-4">
      <button onClick={toggleLight} className="focus:outline-none">
        {!isLightOn ? (
          <HoverableIcon src={light} alt="light" />
        ) : (
          <HoverableIcon src={dark} alt="dark" />
        )}
      </button>
    </div>
  );
}
