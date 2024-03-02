import React, { useEffect, useState } from "react";
import arrow from "../../public/chat/arrow_upward.svg";
import mic from "../../public/chat/mic.svg";
import { useThemeColor } from "@/config/themes";
import IconButton from "@/utils/icon-button";
import { invoke } from "@tauri-apps/api/tauri";

export default function ChatFooter(): JSX.Element {
  const [inputValue, setInputValue] = useState<string>("");

  const handleInputChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInputValue(event.target.value);
  };

  const handleEnterSubmit = (
    event: React.KeyboardEvent<HTMLTextAreaElement>
  ) => {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      submitInput();
      setInputValue("");
    }
  };

  useEffect(() => {
    const textarea = document.getElementById("textarea") as HTMLTextAreaElement;
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
  }, []);

  const backgroundColor = useThemeColor("background");
  const inputColor = useThemeColor("input");

  const submitInput = () => {
    invoke("speak_gpt_from_frontend", { prompt: inputValue });
  };

  return (
    <div style={{ backgroundColor }} className="w-full bg-opacity-60 px-1">
      <div className="max-w-[460px] mx-auto pt-1.5 pb-2  flex  ">
        <div
          style={{ backgroundColor: inputColor }}
          className="max-w-[420px] mx-auto  opacity-60 rounded-md w-full flex justify-between"
        >
          <textarea
            style={{
              resize: "none",
              overflow: "hidden",
              outline: "none",
              flex: "1",
              backgroundColor: inputColor,
            }}
            className="p-2.5 w-full text-sm rounded-lg bg-zinc-950 "
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
