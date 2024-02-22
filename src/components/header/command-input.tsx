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
  const backgroundColor = useThemeColor("background");

  return (
    <AnimatePresence>
      <div className="flex justify-center flex-grow pl-12">
        {!isMobile && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.3 }}
            className={`w-full bg-opacity-80 rounded-md m-1 backdrop-blur-sm ${
              isInputFocused ? "border" : ""
            }`}
            style={isInputFocused ? { borderColor: backgroundColor } : {}}
          >
            <input
              type="text"
              style={{ backgroundColor: inputColor }}
              className="w-full opacity-60 caret-transparent py-2 rounded-md px-4 text-xs bg-transparent text-zinc-500 placeholder-zinc-400 text-center backdrop-blur-lg placeholder-opacity-50 outline-none focus:outline-none"
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
