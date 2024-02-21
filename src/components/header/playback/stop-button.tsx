import React from "react";
import stop from "../../../../public/chat/stop.svg";
import PlaybackButton from "./playback-icon-button";

interface StopButtonProps {
  onClick: () => void;
}

const StopButton: React.FC<StopButtonProps> = ({ onClick }) => (
  <PlaybackButton icon={{ src: stop, alt: "stop" }} onClick={onClick} />
);

export default StopButton;
