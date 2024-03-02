"use client";
import React, { useEffect, useState } from "react";
import useNexus from "@/store";
import ChatMessage from "@/components/chat/chat-message";
import ResponseMessage from "@/components/chat/chat-response-message";
import { Divider } from "@mantine/core";

export default function HomePage() {
  const viewHeight = useNexus((state) => state.viewHeight);
  const viewWidth = useNexus((state) => state.viewWidth);

  return (
    <div className="flex h-full mt-10 max-w-[580px] mx-auto ">
      <div className="flex-1 px-4 text-zinc-400 mt-1.5 gap-2">
        {/* <ChatMessage />
        <div className=" my-4 ">
          <Divider />
        </div>
        <ResponseMessage /> */}
      </div>
    </div>
  );
}
