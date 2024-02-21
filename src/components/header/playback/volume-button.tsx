// VolumeButton.tsx
import React from "react";
import volume_up from "../../../../public/chat/volume_up.svg";
import volume_mute from "../../../../public/chat/volume_mute.svg";
import PlaybackButton from "./playback-icon-button";

interface VolumeButtonProps {
  isMuted: boolean;
  onClick: () => void;
}

const VolumeButton: React.FC<VolumeButtonProps> = ({ isMuted, onClick }) => (
  <PlaybackButton
    icon={{
      src: isMuted ? volume_mute : volume_up,
      alt: isMuted ? "volume mute" : "volume up",
    }}
    onClick={onClick}
  />
);

export default VolumeButton;
