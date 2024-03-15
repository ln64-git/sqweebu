import { useThemeColor } from "@/config/themes";
import React from "react";

const ChatMessage: React.FC<ChatEntry> = ({ body, timestamp }) => {
  const inputColor = useThemeColor("overlay");
  const textSecondary = useThemeColor("textSecondary");

  return (
    <div>
      <div className="flex items-start gap-2.5">
        <div className="flex flex-col gap-1 w-full max-w-[320px]">
          <div className="flex items-center space-x-2 rtl:space-x-reverse"></div>
          <div
            style={{ backgroundColor: inputColor }}
            className="flex flex-col leading-1.5 p-4 opacity-60 rounded-e-xl rounded-es-xl "
          >
            <p
              className="text-sm font-normal X"
              style={{ color: textSecondary }}
            >
              {timestamp + " - " + body}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ChatMessage;
