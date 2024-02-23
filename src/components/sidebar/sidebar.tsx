import React, { useState, useEffect } from "react";
import HoverableIcon from "@/utils/hoverable-icon";
import LightSwitch from "./light-switch";
import useNexus from "@/store";
import { useThemeColor } from "@/config/themes";
import settings from "../../../public/sidebar/settings.svg";
import keyboard from "../../../public/sidebar/keyboard.svg";
import command from "../../../public/sidebar/command.svg";
import voice from "../../../public/sidebar/voice.svg";
import model from "../../../public/sidebar/model.svg";
import chat from "../../../public/sidebar/chat.svg";
import add from "../../../public/sidebar/add.svg";

export default function SideBar() {
  const sidebar = useNexus((state) => state.sidebar);
  const viewHeight = useNexus((state) => state.viewHeight);

  const [isShort1, setIsShort1] = useState(false);
  const [isShort2, setIsShort2] = useState(false);
  const [isShort3, setIsShort3] = useState(false);
  const [isShort4, setIsShort4] = useState(false);
  const [isShort5, setIsShort5] = useState(false);
  const [isShort6, setIsShort6] = useState(false);

  useEffect(() => {
    setIsShort1(viewHeight < 350);
    setIsShort2(viewHeight < 325);
    setIsShort3(viewHeight < 290);
    setIsShort4(viewHeight < 250);
    setIsShort5(viewHeight < 215);
    setIsShort6(viewHeight < 170);
  }, [viewHeight]);

  const overlayColor = useThemeColor("overlay");

  return (
    <>
      {sidebar && (
        <div
          style={{ backgroundColor: overlayColor }}
          className="md:block min-w-[61px] backdrop-blur-md opacity-60 flex flex-col justify-between mt-[40px]"
        >
          <div className="flex flex-col justify-between items-center pt-2.5 h-full">
            <div className="flex flex-col justify-start items-center gap-3 pt-1 w-full h-[134px] pb-[20px]  flex-1">
              <>
                <div className={!isShort6 ? "" : "hidden"}>
                  <HoverableIcon src={add} alt="add" />
                </div>
                <div className={!isShort5 ? "" : "hidden"}>
                  <HoverableIcon src={chat} alt="chat" />
                </div>
                <div className={!isShort4 ? "" : "hidden"}>
                  <HoverableIcon src={model} alt="model" />
                </div>
                <div className={!isShort3 ? "" : "hidden"}>
                  <HoverableIcon src={voice} alt="voice" />
                </div>
              </>
            </div>
            <div className=" sticky bottom-0 left-0 flex flex-col ">
              <div className="flex flex-col justify-between items-center gap-3 w-full p-2  ">
                <div className={!isShort1 ? "" : "hidden"}>
                  <HoverableIcon src={command} alt="command" />
                </div>
                <div className={!isShort2 ? "" : "hidden"}>
                  <HoverableIcon src={keyboard} alt="keyboard" />
                </div>
                <div className="h-[22px]">
                  <HoverableIcon src={settings} alt="settings" />
                </div>
              </div>
              <div className="pt-[2px] h-[54px]">
                <LightSwitch />
              </div>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
