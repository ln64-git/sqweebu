"use client";
import { useState } from "react";
import HoverableIcon from "@/utils/hoverable-icon";
import light from "../../../public/sidebar/light.svg";
import dark from "../../../public/sidebar/dark.svg";
import useInterfaceStore from "@/interface-store";
import userSettingsStore from "@/user-settings-store";

export default function LightSwitch() {
  const [isLightOn, setIsLightOn] = useState(true);
  const setDarkMode = useInterfaceStore((state) => state.setDarkMode);
  const setCurrentUserDarkMode = userSettingsStore(
    (state) => state.setCurrentUserDarkMode
  );

  const toggleLight = () => {
    setDarkMode(!isLightOn);
    setCurrentUserDarkMode(!isLightOn);
    setIsLightOn((prev) => !prev);
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
