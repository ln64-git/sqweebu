import React, { useEffect, useRef, useState } from "react";
import ChatMessage from "./chat-message";
import ResponseMessage from "./chat-response-message";

export default function ChatMain() {
  return (
    <div className="flex-1 px-4 py-2  text-zinc-400">
      {/* <ChatMessage />
      <Divider />
      <ResponseMessage /> */}
    </div>
  );
}

const Divider = () => {
  return <hr className="border-t border-zinc-800 my-4" />;
};
