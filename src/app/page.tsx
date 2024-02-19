"use client";
import ChatBox from "@/components/chat-box";
import PlaybackControls from "@/components/playback-controls";
import { Button } from "@/components/ui/button";
import Welcome from "@/components/welcome";
import { invoke } from "@tauri-apps/api";

export default function Home() {
  return (
    <main className="h-screen flex flex-col">
      <PlaybackControls />
      <Welcome />
      <ChatBox />
    </main>
  );
}
