import React from "react";
import rewind from "../../../../public/chat/fast_rewind.svg";
import PlaybackButton from "./playback-icon-button";

interface RewindButtonProps {
  onClick: () => void;
}

const RewindButton: React.FC<RewindButtonProps> = ({ onClick }) => (
  <PlaybackButton icon={{ src: rewind, alt: "rewind" }} onClick={onClick} />
);

export default RewindButton;
