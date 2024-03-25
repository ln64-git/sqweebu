import React from "react";
import { ChatEntry } from "@/app/page";
import { useTheme } from "../utils/theme-provider";

// Predefined array of colors
const colors = ["#E91E63", "#9C27B0", "#3F51B5", "#4CAF50", "#FF9800"];

const ResponseMessage: React.FC<ChatEntry> = ({ content }) => {
  const { theme } = useTheme();
  const textPrimary = theme.textPrimary;

  return (
    <div className="mx-2 text-sm">
      {content.map((text, index) => (
        // Use a span for inline display, and add a space after each piece of content
        <React.Fragment key={index}>
          <span style={{ color: colors[index % colors.length] || textPrimary }}>
            {text}
          </span>
          {index < content.length - 1 && " "}{" "}
          {/* Conditionally add space except after the last item */}
        </React.Fragment>
      ))}
    </div>
  );
};

export default ResponseMessage;
