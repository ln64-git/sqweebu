import React from "react";
import PlaybackControls from "./playback/playback-controls";
import CommandInput from "./command-input";
import BurgerButton from "./burger-button";
import { useThemeColor } from "@/config/themes";

export default function Header() {
  const backgroundColor = useThemeColor("overlay");

  return (
    <div
      className="sticky top-0 w-full pl-6 h-[40px] backdrop-blur-md z-10 flex justify-between items-center px-4"
      style={{ backdropFilter: "blur(10px)" }}
    >
      <div
        className="absolute inset-0 opacity-60 pt-10 "
        style={{
          backgroundColor,
          backdropFilter: "blur(10px)",
        }}
      />
      <BurgerButton />
      <CommandInput />
      <PlaybackControls />
    </div>
  );
}
