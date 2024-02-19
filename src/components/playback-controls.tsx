import React, { useEffect, useState } from "react";
import { Button } from "./ui/button";
import play_arrow from "../../public/playback/play_arrow.svg";
import pause from "../../public/playback/pause.svg";
import stop from "../../public/playback/stop.svg";
import tune from "../../public/playback/tune.svg";
import fast_forward from "../../public/playback/fast_forward.svg";
import fast_rewind from "../../public/playback/fast_rewind.svg";
import Image from "next/image";
import { invoke } from "@tauri-apps/api";

export default function PlaybackControls() {
  const [isPlaying, setIsPlaying] = useState(false);

  useEffect(() => {
    // TODO - get sink and sync it with isPlaying
  });

  const togglePlayPause = () => {
    if (isPlaying) {
      handlePause();
    } else {
      handleResume();
    }
    setIsPlaying(!isPlaying);
  };

  const handleResume = () => {
    invoke("resume_playback_from_frontend")
      .then((response) => console.log(response))
      .catch((error) => console.error(error));
  };
  const handlePause = () => {
    invoke("pause_playback_from_frontend")
      .then((response) => console.log(response))
      .catch((error) => console.error(error));
  };
  const handleStop = () => {
    invoke("stop_playback_from_frontend")
      .then((response) => console.log(response))
      .catch((error) => console.error(error));
  };
  const handleFastForward = () => {
    invoke("fast_forward_playback_from_frontend")
      .then((response) => console.log(response))
      .catch((error) => console.error(error));
  };

  return (
    <div className="flex justify-between items-center px-4 py-2">
      <Button size="icon">
        <Image src={tune} alt="Tune" />
      </Button>
      <div className="flex justify-center items-center w-full gap-1">
        <Button size="icon">
          <Image src={fast_rewind} alt="Fast Rewind" />
        </Button>
        <Button size="icon" onClick={togglePlayPause}>
          <Image
            src={isPlaying ? pause : play_arrow}
            alt={isPlaying ? "Pause" : "Play"}
          />
        </Button>
        <Button onClick={handleFastForward} size="icon">
          <Image src={fast_forward} alt="Fast Forward" />
        </Button>
      </div>
      <Button onClick={handleStop} size="icon">
        <Image src={stop} alt="Stop" />
      </Button>
    </div>
  );
}
