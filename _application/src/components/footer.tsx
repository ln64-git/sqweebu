import React, { useEffect, useState, useRef } from "react";
import arrow from "../../public/chat/arrow_upward.svg";
import mic from "../../public/chat/mic.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { useTheme } from "./utils/theme-provider";
import IconButton from "./utils/icon-button";
import { useCommandStore } from "@/store/command-store";

export default function ChatFooter(): JSX.Element {
  const [inputValue, setInputValue] = useState<string>("");
  const { theme } = useTheme();
  const backgroundColor = theme.background;
  const inputColor = theme.input;
  const commandMode = useCommandStore((state) => state.mode);
  const textareaRef = useRef<HTMLTextAreaElement>(null); // Ref for textarea

  const handleInputChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInputValue(event.target.value);
  };

  const handleEnterSubmit = (event: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      submitInput();
      setInputValue("");
    }
  };

  const submitInput = () => {
    let commandWithSpaces = commandMode.replace(/_/g, " ");
    let finalCommand = commandWithSpaces + "" + inputValue;
    invoke("process_input_from_frontend", {
      text: finalCommand,
    });
  };

  useEffect(() => {
    // Function to adjust the textarea height
    const adjustTextareaHeight = () => {
      if (!textareaRef.current) return;
      textareaRef.current.style.height = "auto";
      textareaRef.current.style.height = textareaRef.current.scrollHeight + "px";
    };

    // Attach input event listener
    const textarea = textareaRef.current;
    textarea?.addEventListener("input", adjustTextareaHeight);
    
    // Manually trigger adjustment in case of initial content
    adjustTextareaHeight();

    // Cleanup event listener on component unmount
    return () => {
      textarea?.removeEventListener("input", adjustTextareaHeight);
    };
  }, ); // Empty dependency array ensures this effect runs only once on mount

  return (
    <div style={{ backgroundColor }} className="w-full bg-opacity-60 px-1">
      <div className="max-w-[460px] mx-auto pt-1.5 pb-2  flex  ">
        <div className="flex justify-between mx-1 items-center cursor-pointer"></div>
        <div
          style={{ backgroundColor: inputColor }}
          className="max-w-[420px] mx-auto  opacity-60 rounded-md w-full flex justify-between"
        >
          <textarea
            ref={textareaRef}
            style={{
              resize: "none",
              overflow: "hidden",
              outline: "none",
              flex: "1",
              backgroundColor: inputColor,
            }}
            className="p-2.5 w-full text-sm rounded-lg bg-zinc-950"
            rows={1}
            spellCheck={true}
            value={inputValue}
            onChange={handleInputChange}
            onKeyDown={handleEnterSubmit}
          ></textarea>
          <div className="flex items-center justify-center pr-2">
            <div className="transition duration-300 ease-in-out hover:bg-zinc-900 rounded-md cursor-pointer flex ">
              <IconButton
                onClick={submitInput}
                icon={{ src: arrow, alt: "arrow", size: 30 }}
              />
            </div>
          </div>
        </div>
        <div className="flex justify-between mx-1 items-center cursor-pointer">
          <IconButton icon={{ src: mic, alt: "mic", size: 40 }} />
        </div>
      </div>
    </div>
  );
}
