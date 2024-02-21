"use client";
import { useState } from "react";
import Image from "next/image";

interface HoverableIconProps {
  src: string;
  alt: string;
}

const HoverableIcon: React.FC<HoverableIconProps> = ({ src, alt }) => {
  const [isHovered, setIsHovered] = useState(false);

  const handleHover = () => {
    setIsHovered(true);
  };

  const handleMouseLeave = () => {
    setIsHovered(false);
  };

  const imageStyle = {
    filter: isHovered ? "brightness(200%)" : "brightness(100%)",
  };

  return (
    <Image
      src={src}
      alt={alt}
      onMouseEnter={handleHover}
      onMouseLeave={handleMouseLeave}
      style={imageStyle}
    />
  );
};

export default HoverableIcon;
