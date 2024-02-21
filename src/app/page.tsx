"use client";
import React, { useEffect, useState } from "react";
import ChatFooter from "@/components/chat/chat-footer";
import ChatMain from "@/components/chat/chat-main";
import SideBar from "@/components/sidebar/sidebar";
import Header from "@/components/header/header";
import useNexus from "@/store";

export default function Home() {
  const [isMobile, setIsMobile] = useState(false);
  const setMobile = useNexus((state) => state.setMobile);
  const sidebar = useNexus((state) => state.sidebar);

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
    <div className="flex flex-col h-screen bg-zinc-900">
      <div className="flex items-center justify-center pb-10">
        <Header />
      </div>
      <div className="flex flex-1 overflow-y-auto">
        {sidebar && (
          <div className="md:block w-16 bg-zinc-950 bg-opacity-60">
            <SideBar />
          </div>
        )}
        <div className="flex-1 flex flex-col">
          <div className="flex-1 max-w-2xl mx-auto overflow-y-auto">
            <ChatMain />
          </div>
          <div>
            <ChatFooter />
          </div>
        </div>
      </div>
    </div>
  );
}
