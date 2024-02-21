import React from "react";
import rewind from "../../../../public/chat/fast_rewind.svg";
import IconButton from "../../../utils/icon-button";

interface RewindButtonProps {
  onClick: () => void;
}

const RewindButton: React.FC<RewindButtonProps> = ({ onClick }) => (
  <IconButton icon={{ src: rewind, alt: "rewind" }} onClick={onClick} />
);

export default RewindButton;
