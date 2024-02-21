import React from "react";
import Image, { StaticImageData } from "next/image";

interface PlaybackButtonProps {
  icon?: {
    src: StaticImageData;
    alt: string;
  };
  onClick?: () => void;
}

const PlaybackButton: React.FC<PlaybackButtonProps> = ({ icon, onClick }) => (
  <button
    className={`w-8 h-8 ${
      icon ? " p-1 bg-opacity-80 rounded-md m-1 backdrop-blur-sm" : ""
    }`}
    onClick={onClick}
    style={{ cursor: onClick ? "pointer" : "default" }}
  >
    {icon ? <Image width={25} src={icon.src} alt={icon.alt} /> : null}
  </button>
);

export default PlaybackButton;
