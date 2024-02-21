// PlayButton.tsx
import React from "react";
import play_arrow from "../../../../public/chat/play_arrow.svg";
import pause from "../../../../public/chat/pause.svg";
import IconButton from "../../../utils/icon-button";

interface PlayButtonProps {
  isPlaying: boolean;
  onClick: () => void;
}

const PlayButton: React.FC<PlayButtonProps> = ({ isPlaying, onClick }) => (
  <IconButton
    icon={{
      src: isPlaying ? pause : play_arrow,
      alt: isPlaying ? "pause" : "play",
    }}
    onClick={onClick}
  />
);

export default PlayButton;
