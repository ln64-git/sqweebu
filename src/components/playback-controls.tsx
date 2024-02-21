import React from "react";
import Image from "next/image";

// Import icons
import fast_rewind from "../../public/chat/fast_rewind.svg";
import play_arrow from "../../public/chat/play_arrow.svg";
import fast_forward from "../../public/chat/fast_forward.svg";
import volume_down from "../../public/chat/volume_down.svg";
import volume_up from "../../public/chat/volume_up.svg";
import volume_mute from "../../public/chat/volume_mute.svg";
import stop from "../../public/chat/stop.svg";

const controls = [
  { src: stop, alt: "stop" },
  { src: fast_rewind, alt: "rewind" },
  { src: play_arrow, alt: "play / pause" },
  { src: fast_forward, alt: "forward" },
  { src: volume_up, alt: "volume" },
];

export default function PlaybackControls() {
  return (
    <div className="flex">
      {controls.map((control, index) => (
        <div
          key={index}
          className="bg-zinc-950 p-1 bg-opacity-80 rounded-md m-1 backdrop-blur-sm"
        >
          <Image width={25} src={control.src} alt={control.alt} />
        </div>
      ))}
    </div>
  );
}
