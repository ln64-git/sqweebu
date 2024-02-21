// ChatFooter Component

import Image from "next/image";
import React, { useRef, useEffect } from "react";
import arrow from "../../../public/chat/arrow_upward.svg";

export default function ChatFooter() {
  const textareaRef = useRef<HTMLTextAreaElement>(null);

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

  return (
    <div className=" w-full bg-zinc-900 px-1 pt-.5">
      <div className="max-w-xl mx-auto">
        <div className="bg-zinc-950 rounded-md m-2 flex justify-between ">
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
          ></textarea>

          <div className="flex items-center justify-center pr-2">
            <Image
              src={arrow}
              width={30}
              height={30}
              alt="Arrow"
              className="transition duration-300 ease-in-out hover:bg-zinc-900 rounded-md cursor-pointer"
            />
          </div>
        </div>
      </div>
    </div>
  );
}
