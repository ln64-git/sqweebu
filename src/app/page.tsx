"use client";
import React from "react";
import ChatFooter from "@/components/chat/chat-footer";
import Header from "@/components/chat/chat-header";
import ChatMain from "@/components/chat/chat-main";

export default function Home() {
  return (
    <div className="flex flex-col h-screen">
      <div className="flex items-center justify-center">
        <Header />
      </div>
      <div className="bg-zinc-900 flex-1 overflow-y-auto">
        <div className="max-w-2xl mx-auto">
          <ChatMain />
        </div>
      </div>
      <div className="flex items-center justify-center">
        <ChatFooter />
      </div>
    </div>
  );
}
