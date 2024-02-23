import React, { useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import useNexus from "@/store";
import { useThemeColor } from "@/config/themes";

export default function CommandInput() {
  const viewWidth = useNexus((state) => state.viewWidth);
  const showCommandInput = viewWidth < 350;
  const [isInputFocused, setIsInputFocused] = useState(false);

  const handleInputFocus = () => setIsInputFocused(true);
  const handleInputBlur = () => setIsInputFocused(false);

  const inputColor = useThemeColor("input");
  const textPrimary = useThemeColor("textPrimary");
  const overlayColor = useThemeColor("overlay");
  const darkMode = useNexus((state) => state.darkMode);
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
              onFocus={handleInputFocus}
              onBlur={handleInputBlur}
            />
            {isInputFocused && (
              <>
                <div
                  style={{ backgroundColor: inputColor }}
                  className="backdrop-blur-lg opacity-[.45] pt-[4px]"
                />
                <div
                  style={{ backgroundColor: inputColor, color: textPrimary }}
                  className="text-xs backdrop-blur-md opacity-[.70] p-2 pl-4 pr-2 rounded-b-md"
                >
                  <CommandBlock
                    overlayColor={overlayColor}
                    alt="speak_clipboard"
                    label="Speak Clipboard"
                  />
                  <CommandBlock
                    overlayColor={overlayColor}
                    alt="speak_system_monitor"
                    label="Speak System Monitor"
                  />
                  <CommandBlock
                    overlayColor={overlayColor}
                    alt="copy_response_to_clipboard"
                    label="Copy Response to clipboard"
                  />
                </div>
              </>
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
}

function CommandBlock({ overlayColor, alt, label }: CommandBlockProps) {
  const flashCommand = useNexus((state) => state.flashCommand);
  function handleClick() {
    flashCommand(alt);
  }
  return (
    <div className="mb-2 relative">
      <div className="p-1 rounded-sm group cursor-pointer ">
        <div
          onClick={handleClick}
          style={{ background: overlayColor }}
          className="invisible absolute left-0 top-0 w-full p-1 py-[5px] rounded-[4px] transition-opacity duration-300 group-hover:visible"
        >
          {label}
        </div>
        <div className="opacity-100">{label}</div>
      </div>
    </div>
  );
}
