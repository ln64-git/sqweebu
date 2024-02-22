"use client";
import React, { useEffect, useState } from "react";
import useNexus from "@/store";
import ChatMessage from "@/components/chat/chat-message";
import ResponseMessage from "@/components/chat/chat-response-message";
import Header from "@/components/header/header";

export default function HomePage() {
  const [isMobile, setIsMobile] = useState(false);
  const setMobile = useNexus((state) => state.setMobile);

  useEffect(() => {
    const checkIsMobile = () => {
      setIsMobile(window.innerWidth < 350);
    };
    checkIsMobile();
    setMobile(isMobile);
    window.addEventListener("resize", checkIsMobile);
    return () => window.removeEventListener("resize", checkIsMobile);
  }, [isMobile, setMobile]);

  return (
    <div className="flex h-full overflow-y-auto  ">
      <div className="flex-1 px-4 text-zinc-400 mt-1.5 ">
        <ChatMessage />
        <Divider />
        <ResponseMessage />
      </div>
    </div>
  );
}

export const Divider = () => {
  return <hr className="border-t border-zinc-800 my-4" />;
};
