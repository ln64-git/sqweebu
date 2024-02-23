import React, { useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import useNexus from "@/store";
import { useThemeColor } from "@/config/themes";
import { Divider } from "@mantine/core";

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
            className="w-full rounded-md m-1"
          >
            <input
              type="text"
              style={{ backgroundColor: inputColor, color: textPrimary }}
              className={`w-full opacity-40 ${
                isInputFocused ? "rounded-t-md" : "rounded-md"
              } caret-transparent py-2 px-4 text-xs  outline-none focus:outline-none ${placeholderText}`}
              placeholder={isInputFocused ? "" : "Command"}
              onFocus={handleInputFocus}
              onBlur={handleInputBlur}
            />
            {isInputFocused && (
              <>
                <div
                  style={{ backgroundColor: inputColor }}
                  className="backdrop-blur-lg opacity-60 pt-[4px]"
                />
                <div
                  style={{ backgroundColor: inputColor, color: textPrimary }}
                  className="text-xs backdrop-blur-md opacity-60 p-2 pl-4 pr-2 rounded-b-md"
                >
                  <div className="mb-2">Speak Clipboard</div>
                  <div className="mb-2">Speak System Monitor</div>
                  <div className="mb-2">Copy Response to clipboard</div>
                </div>
              </>
            )}
          </motion.div>
        )}
      </div>
    </AnimatePresence>
  );
}
