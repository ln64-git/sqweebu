import React from "react";
import { useTheme } from "../utils/theme-provider";
import formatTimestamp from "@/utils/format-timestamp";
import { ChatEntry } from "@/app/page";

const ChatMessage = (message: ChatEntry) => {
  const { theme } = useTheme();
  const inputColor = theme.overlay;
  const textSecondary = theme.textSecondary;
  return (
    <div>
      <div className="flex items-start gap-2.5">
        <div className="flex flex-col gap-1 w-full max-w-[320px]">
          <div
            style={{ backgroundColor: inputColor }}
            className="flex flex-col leading-1.5 p-4 opacity-60 rounded-e-xl rounded-es-xl "
          >
            <p style={{ color: textSecondary, fontSize: "8px" }}>
              {formatTimestamp(message.timestamp)}
            </p>
            <p className="text-sm font-normal" style={{ color: textSecondary }}>
              {message.content}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ChatMessage;
