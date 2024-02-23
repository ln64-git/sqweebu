"use client";
import React, { useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import useNexus from "@/store";
import { useThemeColor } from "@/config/themes";

export default function CommandInput() {
  const viewWidth = useNexus((state) => state.viewWidth);
  const showCommandInput = viewWidth < 350;
  const [isInputFocused, setIsInputFocused] = useState(false);

  const handleInputFocus = () => {
    setIsInputFocused(true);
  };

  const handleInputBlur = () => {
    setIsInputFocused(false);
  };

  const inputColor = useThemeColor("input");
  const textPrimary = useThemeColor("textPrimary");
  const darkMode = useNexus((state) => state.darkMode);

  var placeholderText = darkMode
    ? "placeholder-zinc-500"
    : "placeholder-zinc-950";

  if (viewWidth > 320) {
    return (
      <AnimatePresence>
        <div className="flex h-[40px]  justify-center flex-grow pl-[12px] pr-[145px] ">
          {!showCommandInput && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.3 }}
              className={`w-full rounded-md m-1`}
            >
              <input
                type="text"
                style={{
                  backgroundColor: inputColor,
                  color: textPrimary,
                }}
                className={
                  isInputFocused
                    ? `border border-stone-800 w-full opacity-40 caret-transparent py-2 rounded-md px-4 text-xs  text-center outline-none focus:outline-none ${placeholderText}`
                    : `w-full opacity-40 caret-transparent py-2 rounded-md px-4 text-xs  text-center outline-none focus:outline-none ${placeholderText}`
                }
                placeholder={isInputFocused ? "" : "Command"}
                onFocus={handleInputFocus}
                onBlur={handleInputBlur}
              />
            </motion.div>
          )}
        </div>
      </AnimatePresence>
    );
  }
}
