import React from "react";
import { useTheme } from "../utils/theme-provider";

interface ChatEntry {
  body: string;
  timestamp: string;
}

const ChatMessage: React.FC<ChatEntry> = ({ body, timestamp }) => {
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
            <p className="text-sm font-normal" style={{ color: textSecondary }}>
              {timestamp + " - " + body}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ChatMessage;
