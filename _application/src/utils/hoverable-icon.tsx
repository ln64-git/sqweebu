import { useState } from "react";
import Image from "next/image";
import useNexus from "@/store";

interface HoverableIconProps {
  src: string;
  alt: string;
  size?: number;
}

const HoverableIcon: React.FC<HoverableIconProps> = ({ src, alt, size }) => {
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

  if (size) {
    return (
      <Image
        src={src}
        alt={alt}
        onMouseEnter={handleHover}
        onMouseLeave={handleMouseLeave}
        onClick={handleClick}
        style={imageStyle}
        width={size}
      />
    );
  } else {
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
  }
};

export default HoverableIcon;
