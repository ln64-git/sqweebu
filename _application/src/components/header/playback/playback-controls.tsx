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
      try {
        const result = await invoke("get_current_sentence");
        console.log(result);
        setIsPlaying(result !== "");
      } catch (error) {
        console.error("Error fetching current sentence:", error);
      }
    };

    // Poll for current sentence every 1 second
    const intervalId = setInterval(getCurrentSentence, 100);

    // Cleanup on component unmount
    return () => clearInterval(intervalId);
  }, []); // Empty dependency array means this runs once on mount

  const toggleMute = () => {
    setIsMuted(!isMuted);
  };

  var display = viewWidth > 200;

  return display ? (
    <div className="flex px-4 w-[156px] fixed right-0 h-[40px]">
      <StopButton onClick={() => {}} />
      <RewindButton onClick={() => {}} />
      <PlayButton isPlaying={isPlaying} />
      <FastForwardButton onClick={() => {}} />
      <VolumeButton isMuted={isMuted} onClick={toggleMute} />
    </div>
  ) : null;
};

export default PlaybackControls;
