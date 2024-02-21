"use client";
import React from "react";
import ChatFooter from "@/components/chat/chat-footer";
import ChatMain from "@/components/chat/chat-main";
import SideBar from "@/components/sidebar/sidebar";
import Header from "@/components/header/header";

export default function Home() {
  return (
    <div className="flex flex-col h-screen bg-zinc-900">
      <div className="flex items-center justify-center pb-10">
        <Header />
      </div>
      <div className="flex flex-1 overflow-y-auto">
        <div className=" md:block w-16 bg-zinc-950 bg-opacity-60">
          <SideBar />
        </div>
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
