"use client";
import React, { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api";
import ChatMessage from "@/components/chat/chat-message";
import ResponseMessage from "@/components/chat/chat-response-message";
import { useDisplayStore } from "@/store/display-store";

export interface ChatEntry {
  source: string;
  timestamp: string;
  content: string;
}

const MessageLog = () => {
  const [messages, setMessages] = useState<ChatEntry[]>([]);
  const messagesEndRef = useRef<HTMLDivElement>(null); // Step 1: Assign a Ref
  const scrollBottom = useDisplayStore((state) => state.scrollBottom);

  useEffect(() => {
    if (scrollBottom) {
      messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
    }
  }, [scrollBottom, messages]);

  useEffect(() => {
    const getData = async () => {
      try {
        const jsonString: string = await invoke("get_chat_updates");
        let data: ChatEntry[] = JSON.parse(jsonString);
        data = data.sort((a, b) => {
          const dateA = new Date(a.timestamp.replace(" at ", " "));
          const dateB = new Date(b.timestamp.replace(" at ", " "));
          return dateA.getTime() - dateB.getTime();
        });
        setMessages(data);
      } catch (error) {
        console.error("Failed to fetch chat updates:", error);
      }
    };
    getData();
    const intervalId = setInterval(getData, 5000);
    return () => clearInterval(intervalId);
  }, []);

  const processedMessages = messages.reduce<ChatEntry[]>((acc, message) => {
    const lastMessage = acc[acc.length - 1];
    if (
      lastMessage &&
      message.source === "gpt" &&
      lastMessage.source === "gpt"
    ) {
      lastMessage.content += "\n" + message.content;
      lastMessage.timestamp = message.timestamp;
    } else {
      acc.push({ ...message });
    }
    return acc;
  }, []);

  return (
    <div className="flex h-full mt-10 max-w-[580px] mx-auto overflow-y-auto">
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
          {/* This div is used as an anchor to scroll into view */}
          <div ref={messagesEndRef} />
        </ul>
      </div>
    </div>
  );
};

export default MessageLog;
