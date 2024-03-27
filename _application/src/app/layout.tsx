"use client";
import React, { useEffect, useRef, useState } from "react";
import { Inter } from "next/font/google";
import "../config/globals.css";
import "@mantine/core/styles.css";
import SideBar from "@/components/sidebar/sidebar";
import Header from "@/components/header/header";
import ChatFooter from "@/components/footer";
import { LayoutProvider } from "@/components/utils/layout-provider";
import { useDisplayStore } from "@/store/display-store";
import { invoke } from "@tauri-apps/api";
import { useCommandStore } from "@/store/command-store";
import ScrollContext from "@/utils/scroll-context";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const scrollRef = useRef<HTMLDivElement>(null);
  const setScrollBottom = useDisplayStore((state) => state.setScrollBottom);

  useEffect(() => {
    const checkIfScrolledToBottom = () => {
      if (!scrollRef.current) return;
      const { scrollTop, scrollHeight, clientHeight } = scrollRef.current;
      const isAtBottom = scrollTop + clientHeight >= scrollHeight - 1; // Allow 1px margin of error
      if (isAtBottom) {
        setScrollBottom(true);
      } else {
        setScrollBottom(false);
      }
    };
    const scrollableElement = scrollRef.current;
    scrollableElement?.addEventListener("scroll", checkIfScrolledToBottom);
    return () =>
      scrollableElement?.removeEventListener("scroll", checkIfScrolledToBottom);
  }, []);

  const command = useCommandStore((state) => state.command);

  useEffect(() => {
    const getSentence = async () => {
      if (command === "stop") {
        let _ = await invoke("stop_playback_from_frontend");
      }
      if (command === "play") {
        let _ = await invoke("play_playback_from_frontend");
      }
      if (command === "pause") {
        let _ = await invoke("pause_playback_from_frontend");
      }
      if (command === "resume") {
        let _ = await invoke("resume_playback_from_frontend");
      }
      if (command === "stop") {
        let _ = await invoke("stop_playback_from_frontend");
      }
    };
    getSentence();
  }, [command]);

  // get_sentence
  return (
    <html lang="en">
      <body className={inter.className}>
        <LayoutProvider>
          <Header />
          <div className=" flex h-full fixed left-0 right-0 bottom-0">
            <SideBar />
            <div className="h-full flex flex-col w-full justify-between mx-auto">
              <ScrollContext.Provider value={scrollRef}>
                <div ref={scrollRef} className="overflow-y-auto  flex-1">
                  {children}
                </div>
              </ScrollContext.Provider>
              <ChatFooter />
            </div>
          </div>
        </LayoutProvider>
      </body>
    </html>
  );
}
