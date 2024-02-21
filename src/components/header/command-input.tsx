"use client";
import React, { useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import useNexus from "@/store";

export default function CommandInput() {
  const isMobile = useNexus((state) => state.isMobile);
  const [isInputFocused, setIsInputFocused] = useState(false);

  const handleInputFocus = () => {
    setIsInputFocused(true);
  };

  const handleInputBlur = () => {
    setIsInputFocused(false);
  };

  return (
    <AnimatePresence>
      <div className="flex justify-center flex-grow">
        {!isMobile && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.3 }}
            className={`bg-zinc-950 w-full bg-opacity-80 rounded-md m-1 backdrop-blur-sm ${
              isInputFocused ? "border-zinc-900 border" : ""
            }`}
          >
            <input
              type="text"
              className="w-full caret-transparent py-2 rounded-md px-4 text-xs bg-transparent text-zinc-500 placeholder-zinc-400 text-center backdrop-blur-lg placeholder-opacity-50 outline-none focus:outline-none"
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
