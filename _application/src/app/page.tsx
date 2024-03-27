"use client";
import React, { useState, useEffect, useRef, useLayoutEffect } from "react";
import { invoke } from "@tauri-apps/api";
import ChatMessage from "@/components/chat/chat-message";
import ResponseMessage from "@/components/chat/chat-response-message";
import { useDisplayStore } from "@/store/display-store";
import { useStore } from "zustand";
import { useScrollContext } from "@/utils/scroll-context";

export interface ChatEntry {
  source: string;
  timestamp: string;
  content: string[];
}

const MessageLog = () => {
  const [messages, setMessages] = useState<ChatEntry[]>([]);
  const [initialLoad, setInitialLoad] = useState(true); // Flag to track the initial load
  const scrollBottom = useDisplayStore((state) => state.scrollBottom);
  const scrollRef = useScrollContext();

  useLayoutEffect(() => {
    if (scrollBottom || initialLoad) {
      if (scrollRef != null) {
        if (scrollRef.current) {
          requestAnimationFrame(() => {
            scrollRef.current?.scrollTo({
              top: scrollRef.current.scrollHeight,
              behavior: "instant",
            });
          });
        }
      }
    }
  }, [scrollBottom, initialLoad, messages]);

  // Handle automatic scrolling
  useEffect(() => {
    if (scrollBottom || initialLoad) {
      if (scrollRef != null) {
        scrollRef.current?.scrollTo({
          top: scrollRef.current.scrollHeight,
          behavior: "instant",
        });
      }
    }
  }, [messages]); // Depend on messages to trigger scroll

  useEffect(() => {
    const getData = async () => {
      try {
        const jsonString: string = await invoke("get_chat_updates");
        const data: ChatEntry[] = JSON.parse(jsonString);
        const sortedData = data.sort((a, b) => {
          const dateA = new Date(a.timestamp.replace(" at ", " "));
          const dateB = new Date(b.timestamp.replace(" at ", " "));
          return dateA.getTime() - dateB.getTime();
        });
        setMessages(sortedData);
        if (initialLoad) {
          setInitialLoad(false); // After initial load, set flag to false
        }
      } catch (error) {
        console.error("Failed to fetch chat updates:", error);
      }
    };
    getData();
    // Set an interval for live updates if needed. Adjust the interval as needed.
    const intervalId = setInterval(getData, 300); // Example: 10 seconds
    return () => clearInterval(intervalId);
  }, []);

  const processedMessages = messages.reduce<ChatEntry[]>(
    (acc, currentMessage) => {
      const lastMessage = acc[acc.length - 1];
      if (
        lastMessage &&
        currentMessage.source === "gpt" &&
        lastMessage.source === "gpt"
      ) {
        // Normalize currentMessage.content to always be an array
        const contentToAdd = Array.isArray(currentMessage.content)
          ? currentMessage.content
          : [currentMessage.content];
        // Now safely spread contentToAdd since it's guaranteed to be an array
        lastMessage.content = [...lastMessage.content, ...contentToAdd];
        lastMessage.timestamp = currentMessage.timestamp;
      } else {
        // For a new message, ensure content is treated as an array
        const newContent = Array.isArray(currentMessage.content)
          ? currentMessage.content
          : [currentMessage.content];
        acc.push({ ...currentMessage, content: newContent });
      }
      return acc;
    },
    []
  );

  return (
    <div className="flex h-full  mt-10 max-w-[580px] mx-auto">
      <div className="flex-1 px-4 text-zinc-400 mt-1.5 gap-2">
        <ul>
          {processedMessages.map((message, index) => (
            <div className="py-1" key={index}>
              {message.source === "user" ? (
                <ChatMessage
                  source={message.source}
                  content={message.content}
                  timestamp={message.timestamp}
                />
              ) : (
                <ResponseMessage
                  source={message.source}
                  content={message.content}
                  timestamp={message.timestamp}
                />
              )}
            </div>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default MessageLog;
