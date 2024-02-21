import React from "react";
import fast_forward from "../../../../public/chat/fast_forward.svg";
import PlaybackButton from "./playback-icon-button";

interface FastForwardButtonProps {
  onClick: () => void;
}

const FastForwardButton: React.FC<FastForwardButtonProps> = ({ onClick }) => (
  <PlaybackButton
    icon={{ src: fast_forward, alt: "fast forward" }}
    onClick={onClick}
  />
);

export default FastForwardButton;
