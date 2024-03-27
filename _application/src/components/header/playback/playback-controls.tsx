import React, { useEffect, useState } from "react";
import StopButton from "./stop-button";
import RewindButton from "./rewind-button";
import PlayButton from "./play-pause-button";
import FastForwardButton from "./fast-forward-button";
import VolumeButton from "./volume-button";
import { useDisplayStore } from "@/store/display-store";
import { invoke } from "@tauri-apps/api";

const PlaybackControls = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [isMuted, setIsMuted] = useState(false);
  const viewWidth = useDisplayStore((state) => state.viewWidth);

  useEffect(() => {
    const getCurrentSentence = async () => {
      const result = await invoke("get_current_sentence");
      console.log(result);
      if (result === "") {
        console.log("true");
        setIsPlaying(false);
      } else {
        console.log("true");
        setIsPlaying(true);
      }
    };
    getCurrentSentence();
  });

  // const togglePlay = () => {
  //   setIsPlaying(!isPlaying);
  // };

  const toggleMute = () => {
    setIsMuted(!isMuted);
  };

  var display = viewWidth > 200;

  if (display) {
    return (
      <div className="flex px-4 w-[156px] fixed right-0 h-[40px]">
        <StopButton onClick={() => {}} />
        <RewindButton onClick={() => {}} />
        <PlayButton isPlaying={isPlaying} />
        <FastForwardButton onClick={() => {}} />
        <VolumeButton isMuted={isMuted} onClick={toggleMute} />
      </div>
    );
  } else {
    return;
  }
};

export default PlaybackControls;
