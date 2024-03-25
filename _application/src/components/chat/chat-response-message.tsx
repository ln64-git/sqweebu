import React from "react";
import { ChatEntry } from "@/app/page";
import { useTheme } from "../utils/theme-provider";

const ResponseMessage: React.FC<ChatEntry> = ({ content }) => {
  const { theme } = useTheme();
  const textPrimary = theme.textPrimary;
  const textSecondary = theme.textSecondary; // Corrected to use textSecondary

  const handleClick = (index: number) => {
    console.log(content[index]);
  };

  return (
    <div className="mx-2 text-sm">
      {content.map((text, index) => (
        <React.Fragment key={index}>
          <span
            onClick={() => handleClick(index)}
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
