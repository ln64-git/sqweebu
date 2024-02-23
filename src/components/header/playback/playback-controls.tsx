"use client";
import React, { useEffect, useState } from "react";
import StopButton from "./stop-button";
import RewindButton from "./rewind-button";
import PlayButton from "./play-pause-button";
import FastForwardButton from "./fast-forward-button";
import VolumeButton from "./volume-button";

const PlaybackControls = () => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [isMuted, setIsMuted] = useState(false);
  const [windowWidth, setWindowWidth] = useState(window.innerWidth);

  const togglePlay = () => {
    setIsPlaying(!isPlaying);
  };

  const toggleMute = () => {
    setIsMuted(!isMuted);
  };

  useEffect(() => {
    function handleResize() {
      setWindowWidth(window.innerWidth);
    }

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  if (windowWidth <= 200) {
    return null;
  }

  return (
    <div className="flex px-4 w-[156px] fixed right-0">
      <StopButton onClick={() => {}} />
      <RewindButton onClick={() => {}} />
      <PlayButton isPlaying={isPlaying} onClick={togglePlay} />
      <FastForwardButton onClick={() => {}} />
      <VolumeButton isMuted={isMuted} onClick={toggleMute} />
    </div>
  );
};

export default PlaybackControls;
