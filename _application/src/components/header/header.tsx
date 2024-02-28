import React from "react";
import PlaybackControls from "./playback/playback-controls";
import CommandInput from "./command-input";
import BurgerButton from "./burger-button";
import { useThemeColor } from "@/config/themes";

export default function Header() {
  const overlayColor = useThemeColor("overlay");

  return (
    <div className="flex w-full  ">
      <div className="sticky top-0 w-full z-10 flex justify-between items-center">
        <div
          className="absolute inset-0 backdrop-blur-md opacity-60 h-full"
          style={{
            backgroundColor: overlayColor,
          }}
        />
        <BurgerButton />
        <CommandInput />
        <PlaybackControls />
      </div>
      <div
        style={{
          background: overlayColor,
          backdropFilter: "blur(10px)",
          opacity: 0.6,
        }}
        className="w-[16px] right-0"
      ></div>
    </div>
  );
}
