"use client";
import settings from "../../../public/sidebar/settings.svg";
import keyboard from "../../../public/sidebar/keyboard.svg";
import command from "../../../public/sidebar/command.svg";
import person from "../../../public/sidebar/person.svg";
import model from "../../../public/sidebar/model.svg";
import chat from "../../../public/sidebar/chat.svg";
import HoverableIcon from "@/utils/hoverable-icon";
import LightSwitch from "./light-switch";
import useNexus from "@/store";

export default function SideBar() {
  const sidebar = useNexus((state) => state.sidebar);

  return (
    <>
      {sidebar && (
        <div className="mt-10 md:block min-w-[61px] bg-zinc-950 bg-opacity-60 flex flex-col justify-between">
          <div className="flex flex-col items-center gap-2 pt-3">
            <HoverableIcon src={chat} alt="chat" />
            <HoverableIcon src={model} alt="model" />
            <HoverableIcon src={person} alt="person" />
            <HoverableIcon src={command} alt="command" />
            <HoverableIcon src={keyboard} alt="keyboard" />
            <HoverableIcon src={settings} alt="settings" />
          </div>
          <div className="pt-4">
            <LightSwitch />
          </div>
        </div>
      )}
    </>
  );
}
