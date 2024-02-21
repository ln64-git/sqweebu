import React, { useEffect, useRef, useState } from "react";
import ChatMessage from "./chat-message";
import ResponseMessage from "./chat-response-message";
import SideBar from "../sidebar/sidebar";

export default function ChatMain() {
  return (
    <div className="flex  ">
      <div className="flex-1bg-red-900 px-4 py-2  text-zinc-400">
        {/* <ChatMessage />
      <Divider />
    <ResponseMessage /> */}
      </div>
    </div>
  );
}

const Divider = () => {
  return <hr className="border-t border-zinc-800 my-4" />;
};
