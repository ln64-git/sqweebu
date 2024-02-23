import React from "react";
import HoverableIcon from "@/utils/hoverable-icon";

interface IconButton {
  icon?: {
    src: string;
    alt: string;
    size?: number;
  };
  onClick?: () => void;
}

const IconButton: React.FC<IconButton> = ({ icon, onClick }) => (
  <button onClick={onClick} style={{ cursor: onClick ? "pointer" : "default" }}>
    {icon ? (
      <HoverableIcon src={icon.src} alt={icon.alt} size={icon.size} />
    ) : null}
  </button>
);

export default IconButton;
