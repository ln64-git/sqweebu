// VolumeButton.tsx
import React from "react";
import volume_up from "../../../../public/chat/volume_up.svg";
import volume_mute from "../../../../public/chat/volume_mute.svg";
import IconButton from "../../../utils/icon-button";

interface VolumeButtonProps {
  isMuted: boolean;
  onClick: () => void;
}

const VolumeButton: React.FC<VolumeButtonProps> = ({ isMuted, onClick }) => (
  <IconButton
    icon={{
      src: isMuted ? volume_mute : volume_up,
      alt: isMuted ? "volume mute" : "volume up",
    }}
    onClick={onClick}
  />
);

export default VolumeButton;
