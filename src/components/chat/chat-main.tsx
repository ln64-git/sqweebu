import React from "react";
import Image from "next/image";
import ChatMessage from "./chat-message";
import ResponseMessage from "./chat-response-message";

export default function ChatMain() {
  return (
    <div className="flex-1 mx-4 p-4  ">
      <ChatMessage />
      <ResponseMessage />
    </div>
  );
}
