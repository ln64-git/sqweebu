"use client";
import ChatMessage from "@/components/chat/chat-message";
import { invoke } from "@tauri-apps/api";
import { useState, useEffect } from "react";

interface ChatMessage {
  timestamp: string;
  body: string;
}

const HomePage = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);

  useEffect(() => {
    const getData = async () => {
      try {
        const jsonString: string = await invoke("get_chat_updates");
        const data = JSON.parse(jsonString);
        console.log(data);
        setMessages(data);
      } catch (error) {
        console.error("Failed to fetch chat updates:", error);
      }
    };
    getData();
    const intervalId = setInterval(getData, 5000);
    return () => clearInterval(intervalId);
  }, []);

  return (
    <div className="flex h-full mt-10 max-w-[580px] mx-auto">
      <div className="flex-1 px-4 text-zinc-400 mt-1.5 gap-2">
        {messages && (
          <div className="mt-4">
            <ul>
              {messages.map((message, index) => (
                <ChatMessage
                  body={message.body}
                  timestamp={message.timestamp}
                />
                // <li key={index}>
                //   <strong>
                //     {new Date(message.timestamp).toLocaleString()}:
                //   </strong>{" "}
                //   {message.body}
                // </li>
              ))}
            </ul>
          </div>
        )}
      </div>
    </div>
  );
};

export default HomePage;
