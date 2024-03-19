"use client";
import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import ChatMessage from "@/components/chat/chat-message";
import ResponseMessage from "@/components/chat/chat-response-message";

export interface ChatEntry {
  source: string;
  timestamp: string;
  content: string;
}

const MessageLog = () => {
  const [messages, setMessages] = useState<ChatEntry[]>([]);

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
    <div className="flex h-full mt-10 max-w-[580px] mx-auto">
      <div className="flex-1 px-4 text-zinc-400 mt-1.5 gap-2">
        <div className="">
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
    </div>
  );
};

export default MessageLog;
