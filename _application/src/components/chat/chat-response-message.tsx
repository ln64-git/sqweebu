import React from "react";
import { ChatEntry } from "@/app/page";
import { useTheme } from "../utils/theme-provider";
import { invoke } from "@tauri-apps/api";

const ResponseMessage = (message: ChatEntry) => {
  const { theme } = useTheme();
  const textPrimary = theme.textPrimary;
  const textSecondary = theme.textSecondary;

  const handleClick = (message: ChatEntry) => {
    const getData = async () => {
      let data = await invoke("get_current_sentence");
      console.log(data);
    };
    getData();
    console.log("clicked");
  };

  return (
    <div className="mx-2 text-sm">
      {message.content.map((text, index) => (
        <React.Fragment key={index}>
          <span
            onClick={() => handleClick(message)}
            className="cursor-pointer"
            style={{
              color: textPrimary,
              transition: "color 300ms ease-in-out", // Smooth transition for hover effect
            }}
            onMouseEnter={(e) => (e.currentTarget.style.color = textSecondary)}
            onMouseLeave={(e) => (e.currentTarget.style.color = textPrimary)}
          >
            {text}
          </span>
        </React.Fragment>
      ))}
    </div>
  );
};

export default ResponseMessage;
