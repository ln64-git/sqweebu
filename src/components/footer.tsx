"use client";

import Image from "next/image";
import React, { useRef, useEffect, useState } from "react";
import arrow from "../../public/chat/arrow_upward.svg";
import mic from "../../public/chat/mic.svg";
import useNexus from "@/store";
import { useThemeColor } from "@/config/themes";

export default function ChatFooter() {
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const handleEnterSubmit = (
    event: React.KeyboardEvent<HTMLTextAreaElement>
  ) => {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      console.log("Submit message:", event.currentTarget.value);
      // Add logic to submit the message here
      event.currentTarget.value = "";
    }
  };

  useEffect(() => {
    const textarea = textareaRef.current;
    if (!textarea) return;
    const adjustTextareaHeight = () => {
      textarea.style.height = "auto";
      textarea.style.height = textarea.scrollHeight + "px";
    };
    textarea.addEventListener("input", adjustTextareaHeight);
    textarea.dispatchEvent(new Event("input"));
    return () => {
      textarea.removeEventListener("input", adjustTextareaHeight);
    };
  }, [textareaRef]);

  const backgroundColor = useThemeColor("background");

  return (
    <div
      style={{ backgroundColor }}
      className="w-full bg-zinc-900 bg-opacity-60 px-1"
    >
      <div className="max-w-xl mx-auto pt-1.5 pb-2 pl-2 flex">
        <div className="bg-zinc-950 rounded-md w-full flex justify-between">
          <textarea
            ref={textareaRef}
            style={{
              resize: "none",
              overflow: "hidden",
              outline: "none",
              flex: "1",
            }}
            className="p-2.5 w-full text-sm rounded-lg bg-zinc-950"
            rows={1}
            spellCheck={true}
            onKeyDown={handleEnterSubmit}
          ></textarea>
          <div className="flex items-center justify-center pr-2">
            <Image
              src={arrow}
              width={30}
              alt="Arrow"
              className="transition duration-300 ease-in-out hover:bg-zinc-900 rounded-md cursor-pointer"
            />
          </div>
        </div>
        <div className="flex justify-between mx-1 items-center">
          <Image src={mic} width={40} alt="mic" />
        </div>
      </div>
    </div>
  );
}
