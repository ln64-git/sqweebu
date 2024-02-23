"use client";
import React, { useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import useNexus from "@/store";
import { useThemeColor } from "@/config/themes";

export default function CommandInput() {
  const isMobile = useNexus((state) => state.isMobile);
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

  return (
    <AnimatePresence>
      <div className="flex justify-center flex-grow pl-[72px] pr-[145px]">
        {!isMobile && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.3 }}
            className={`w-full  rounded-md m-1  ${
              isInputFocused ? "border-stone-600 border" : ""
            }`}
            // style={isInputFocused ? { borderColor: backgroundColor } : {}}
          >
            <input
              type="text"
              style={{
                backgroundColor: inputColor,
                color: textPrimary,
              }}
              className={`w-full opacity-60 caret-transparent py-2 rounded-md px-4 text-xs   text-center outline-none focus:outline-none ${placeholderText}`}
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
