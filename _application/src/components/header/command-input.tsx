import React, { useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { useDisplayStore } from "@/store/display-store";
import { useThemeStore } from "@/store/theme-store";
import { useCommandStore } from "@/store/command-store";
import { useTheme } from "../utils/theme-provider";
import IconButton from "../utils/icon-button";
import check from "../../../public/check.svg";

export default function CommandInput() {
  const viewWidth = useDisplayStore((state) => state.viewWidth);
  const showCommandInput = viewWidth < 350;
  const [isInputFocused, setIsInputFocused] = useState(false);

  // const handleInputFocus = () => setIsInputFocused(true);
  // const handleInputBlur = () => setIsInputFocused(false);
  const { theme } = useTheme();
  const inputColor = theme.input;
  const textPrimary = theme.textPrimary;
  const overlayColor = theme.overlay;

  const darkMode = theme.darkMode;
  const placeholderText = darkMode
    ? "placeholder-zinc-500"
    : "placeholder-zinc-950";

  if (viewWidth <= 320) return null;
  return (
    <AnimatePresence>
      <div className="flex h-[40px] justify-center flex-grow pl-[13px] pr-[145px]">
        {!showCommandInput && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.3 }}
            className="w-full rounded-md m-1 "
          >
            <input
              type="text"
              style={{ backgroundColor: inputColor, color: textPrimary }}
              className={`w-full opacity-40 ${
                isInputFocused ? "rounded-t-md" : "rounded-md"
              }  py-2 px-5 text-xs  outline-none focus:outline-none ${placeholderText}`}
              placeholder={isInputFocused ? "" : "Command"}
              onClick={() => {
                setIsInputFocused(true);
              }}
              onBlur={() => {
                setTimeout(() => setIsInputFocused(false), 100);
              }}
            />

            {isInputFocused && (
              <ul>
                <div
                  style={{ backgroundColor: inputColor }}
                  className="backdrop-blur-lg z-50 opacity-[.45] pt-[4px]"
                />
                <div
                  style={{ backgroundColor: inputColor, color: textPrimary }}
                  className=" text-xs backdrop-blur-md opacity-[.70] p-2 pl-4 pr-2 rounded-b-md"
                >
                  <CommandBlock
                    overlayColor={overlayColor}
                    alt="speak_clipboard"
                    label="Speak Clipboard"
                    checkbox={false}
                  />
                  <CommandBlock
                    overlayColor={overlayColor}
                    alt="speak_system_monitor"
                    label="Speak System Monitor"
                    checkbox={false}
                  />
                  <CommandBlock
                    overlayColor={overlayColor}
                    alt="copy_response_to_clipboard"
                    label="Copy Response to clipboard"
                    checkbox={false}
                  />

                  <CustomDivider />
                  <CommandBlock
                    overlayColor={overlayColor}
                    alt="speak_gpt"
                    label="Speak GPT"
                    checkbox={true}
                  />
                </div>
              </ul>
            )}
          </motion.div>
        )}
      </div>
    </AnimatePresence>
  );
}

interface CommandBlockProps {
  overlayColor: string;
  alt: string;
  label: string;
  checkbox: boolean;
}

function CommandBlock({
  overlayColor,
  alt,
  label,
  checkbox = false,
}: CommandBlockProps) {
  const commandMode = useCommandStore((state) => state.mode);
  const setMode = useCommandStore((state) => state.setMode);
  const isChecked = commandMode === alt;

  function handleClick() {
    console.log("clicked");
    if (checkbox === true && commandMode != alt) {
      setMode(alt);
    } else if (checkbox === true) {
      setMode("");
    }
  }

  return (
    <div onClick={handleClick} className="relative ">
      <div className="p-1 rounded-sm group cursor-pointer">
        <div
          style={{ background: overlayColor }}
          className="  invisible absolute flex items-center left-0 top-0 w-full p-1 py-[5px] rounded-[4px] transition-opacity duration-300 group-hover:visible"
        >
          {checkbox && <CheckBox checked={isChecked} />}
          {label}
        </div>
        <div className="opacity-100 flex items-center">
          {checkbox && <CheckBox checked={isChecked} />}
          {label}
        </div>
      </div>
    </div>
  );
}

const CheckBox = ({ checked }: { checked: boolean }) => {
  const { theme } = useTheme();
  const overlayColor = theme.overlay;
  return (
    <div
      style={{ background: overlayColor }}
      className="rounded-md flex justify-center items-center mr-2 w-5 h-5"
    >
      {checked && <IconButton icon={{ src: check, alt: "check", size: 15 }} />}
    </div>
  );
};

const CustomDivider = () => {
  const { theme } = useTheme();
  const overlayColor = theme.overlay;
  return (
    <div className="relative flex py-2 items-center">
      <div
        style={{ borderColor: overlayColor }}
        className="flex-grow border-t "
      ></div>
      <span className="flex-shrink "></span>
      <div
        style={{ borderColor: overlayColor }}
        className="flex-grow border-t "
      ></div>
    </div>
  );
};
