import React, { useState } from "react";

import settings from "../../../public/sidebar/settings.svg";
import keyboard from "../../../public/sidebar/keyboard.svg";
import command from "../../../public/sidebar/command.svg";
import person from "../../../public/sidebar/person.svg";
import model from "../../../public/sidebar/model.svg";
import HoverableIcon from "@/utils/hoverable-icon";

export default function SideBar() {
  return (
    <div className="flex flex-col justify-between h-full">
      <div className="flex flex-col items-center gap-2 pt-3">
        <HoverableIcon src={model} alt="model" />
        <HoverableIcon src={person} alt="person" />
        <HoverableIcon src={command} alt="command" />
        <HoverableIcon src={keyboard} alt="keyboard" />
        <HoverableIcon src={settings} alt="settings" />
      </div>
    </div>
  );
}
