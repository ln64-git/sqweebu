import React from "react";
import stop from "../../../../public/chat/stop.svg";
import IconButton from "../../../utils/icon-button";

interface StopButtonProps {
  onClick: () => void;
}

const StopButton: React.FC<StopButtonProps> = ({ onClick }) => (
  <IconButton icon={{ src: stop, alt: "stop" }} onClick={onClick} />
);

export default StopButton;
