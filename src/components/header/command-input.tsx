import React, { useState, useEffect } from "react";
import { AnimatePresence, motion } from "framer-motion";

export default function CommandInput() {
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    const checkIsMobile = () => {
      setIsMobile(window.innerWidth < 350);
    };

    checkIsMobile();
    window.addEventListener("resize", checkIsMobile);

    return () => window.removeEventListener("resize", checkIsMobile);
  });

  return (
    <AnimatePresence>
      <div className="flex justify-center flex-grow">
        {!isMobile && (
          <motion.div
            initial={{ opacity: 0 }} // Initial opacity when component mounts
            animate={{ opacity: 1 }} // Animate opacity to 1 to show the input
            transition={{ duration: 0.3 }} // Transition duration for fade animation
            className="bg-zinc-950 w-full bg-opacity-80 rounded-md m-1 backdrop-blur-sm"
          >
            <input
              type="text"
              className="w-full caret-transparent py-2 rounded-md px-4 text-xs bg-transparent text-zinc-500 placeholder-zinc-400 text-center backdrop-blur-lg placeholder-opacity-50 outline-none focus:outline-none"
              placeholder="Command"
            />
          </motion.div>
        )}
      </div>
    </AnimatePresence>
  );
}
