"use client";
import { useState } from "react";
import HoverableIcon from "@/utils/hoverable-icon";
import light from "../../../public/sidebar/light.svg";
import dark from "../../../public/sidebar/dark.svg";
import useNexus from "@/store";

export default function LightSwitch() {
  const [isLightOn, setIsLightOn] = useState(true);
  const setDarkMode = useNexus((state) => state.setDarkMode);

  const toggleLight = () => {
    setDarkMode(!isLightOn);
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
