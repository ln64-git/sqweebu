import { useState } from "react";
import Image from "next/image";
import useNexus from "@/store";

interface HoverableIconProps {
  src: string;
  alt: string;
}

const HoverableIcon: React.FC<HoverableIconProps> = ({ src, alt }) => {
  const [isHovered, setIsHovered] = useState(false);
  const flashCommand = useNexus((state) => state.flashCommand);

  const handleClick = () => {
    flashCommand(alt);
  };

  const handleHover = () => {
    setIsHovered(true);
  };

  const handleMouseLeave = () => {
    setIsHovered(false);
  };

  const imageStyle = {
    filter: isHovered ? "brightness(200%)" : "brightness(100%)",
    cursor: "pointer",
  };

  return (
    <Image
      src={src}
      alt={alt}
      onMouseEnter={handleHover}
      onMouseLeave={handleMouseLeave}
      onClick={handleClick}
      style={imageStyle}
    />
  );
};

export default HoverableIcon;
