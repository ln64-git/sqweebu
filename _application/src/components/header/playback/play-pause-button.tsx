// PlayButton.tsx
import React from "react";
import play_arrow from "../../../../public/chat/play_arrow.svg";
import pause from "../../../../public/chat/pause.svg";
import IconButton from "../../utils/icon-button";
import { invoke } from "@tauri-apps/api";

interface PlayButtonProps {
  isPlaying: boolean;
}

const onClick = async (isPlaying: boolean) => {
  const playback = isPlaying;
  console.log(isPlaying);
  if (playback) {
    console.log("Clicked");
    invoke("pause_playback_from_frontend");
  } else {
    console.log("Clicked");
    invoke("resume_playback_from_frontend");
  }
};

const PlayButton: React.FC<PlayButtonProps> = ({ isPlaying }) => (
  <IconButton
    icon={{
      src: isPlaying ? pause : play_arrow,
      alt: isPlaying ? "pause" : "play",
    }}
    onClick={() => {
      onClick(isPlaying);
    }}
  />
);

export default PlayButton;
