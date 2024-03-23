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

  const [activeSentence, setActiveSentence] = useState("");

  useEffect(() => {
    const getSentence = async () => {
      let updatedSentence: string = await invoke("get_active_sentence");
      setActiveSentence(updatedSentence);
    };
    getSentence();
    const intervalId = setInterval(getSentence, 5000);
    console.log(activeSentence);
    return () => clearInterval(intervalId);
  });

  return (
    <html lang="en">
      <body className={inter.className}>
        <LayoutProvider>
          <Header />
          <div className="flex h-full fixed left-0 right-0 bottom-0">
            <SideBar />
            <div className="h-full flex flex-col w-full justify-between mx-auto">
              <div ref={scrollRef} className="overflow-y-auto flex-1">
                {children}
              </div>
              <ChatFooter />
            </div>
          </div>
        </LayoutProvider>
      </body>
    </html>
  );
}
