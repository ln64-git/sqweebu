"use client";
import React, { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api";
import ChatMessage from "@/components/chat/chat-message";
import ResponseMessage from "@/components/chat/chat-response-message";
import { useDisplayStore } from "@/store/display-store";

export interface ChatEntry {
  source: string;
  timestamp: string;
  content: string[];
}

const MessageLog = () => {
  let [currentSentence, setCurrentSentence] = useState<String>("");
  let [messages, setMessages] = useState<ChatEntry[]>([]);
  let [liveMessages, setLiveMessages] = useState<ChatEntry[]>([]);
  let messagesEndRef = useRef<HTMLDivElement>(null); // Step 1: Assign a Ref
  let scrollBottom = useDisplayStore((state) => state.scrollBottom);

  useEffect(() => {
    const getData = async () => {
      let data: string = await invoke("get_current_sentence");
      setCurrentSentence(data);
    };
    getData();
    console.log(currentSentence);
  });

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
  });

  useEffect(() => {
    if (messages.length > 0) {
      const latestChatEntry = messages[messages.length - 1];
      if (
        latestChatEntry.source === "user" ||
        currentSentence === "No Currently Selected Sentence"
      ) {
        setLiveMessages(messages);
      }

      // This function finds the index of the message that matches the currentSentence.
      // It returns -1 if no match is found.
      const findMatchingSentenceIndex = () => {
        for (let i = messages.length - 1; i >= 0; i--) {
          const entry = messages[i];
          if (
            entry.source === "gpt" &&
            entry.content.includes(currentSentence.toString())
          ) {
            return i;
          }
        }
        return -1; // Return -1 if no match is found
      };

      const matchingIndex = findMatchingSentenceIndex();

      // If a matching sentence is found in the backlog, update liveMessages to include
      // messages up to and including the matching message.
      if (matchingIndex !== -1) {
        // Set liveMessages to include all messages up to and including the match
        const updatedLiveMessages = messages.slice(0, matchingIndex + 1);
        setLiveMessages(updatedLiveMessages);
        console.log(
          "Match found in backlog, updating liveMessages up to currentSentence"
        );
      } else {
        // If there's no match but the latest message is from the user or there's no currently speaking sentence,
        // consider setting liveMessages based on specific app logic. For example, you might not update liveMessages
        // at all if you only want to update when there's a match.
      }
    }
  }, [messages, currentSentence]); // Depend on messages and currentSentence

  const processedMessages = liveMessages.reduce<ChatEntry[]>(
    (acc, currentMessage) => {
      const lastMessage = acc[acc.length - 1];
      if (
        lastMessage &&
        currentMessage.source === "gpt" &&
        lastMessage.source === "gpt"
      ) {
        const contentToAdd = Array.isArray(currentMessage.content)
          ? currentMessage.content
          : [currentMessage.content];
        lastMessage.content = [...lastMessage.content, ...contentToAdd];
        lastMessage.timestamp = currentMessage.timestamp;
      } else {
        const newContent = Array.isArray(currentMessage.content)
          ? currentMessage.content
          : [currentMessage.content];
        acc.push({ ...currentMessage, content: newContent });
      }
      return acc;
    },
    []
  );

  useEffect(() => {
    if (scrollBottom) {
      messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
    }
  }, [scrollBottom, messages]);

  return (
    <div className="flex h-full mt-10 max-w-[580px] mx-auto overflow-y-auto">
      <div className="flex-1 px-4 text-zinc-400 mt-1.5 gap-2">
        <ul>
          {processedMessages.map((message, index) => (
            <div className="py-1" key={index}>
              {message.source === "user" ? (
                <ChatMessage {...message} />
              ) : (
                <ResponseMessage {...message} />
              )}
            </div>
          ))}
          <div ref={messagesEndRef} />
        </ul>
      </div>
    </div>
  );
};

export default MessageLog;
