import Image from "next/image";
import React from "react";

import settings from "../../../public/sidebar/settings.svg";
import keyboard from "../../../public/sidebar/keyboard.svg";
import command from "../../../public/sidebar/command.svg";
import person from "../../../public/sidebar/person.svg";
import model from "../../../public/sidebar/model.svg";

export default function SideBar() {
  return (
    <div className="flex flex-col justify-between  h-full">
      <div className="flex flex-col items-center gap-2 pt-3">
        <Image src={model} alt="settings" width={35} />
        <Image src={person} alt="settings" width={35} />
        <Image src={command} alt="settings" width={35} />
        <Image src={keyboard} alt="settings" width={38} />
        <Image src={settings} alt="settings" width={35} />
      </div>
      {/* <div className="flex flex-col items-center gap-2 pb-3">
        <Image src={settings} alt="settings" width={35} />
      </div> */}
    </div>
  );
}
