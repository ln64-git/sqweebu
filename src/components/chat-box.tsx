import React, { useState } from "react";
import { Button } from "./ui/button";
import Image from "next/image";
import { Input } from "@/components/ui/input";
import arrow from "../../public/arrow_upward.svg";
import { invoke } from "@tauri-apps/api";

export default function ChatBox() {
  const [prompt, setPrompt] = useState("");

  const handleSpeakOllama = () => {
    invoke("speak_ollama_from_frontend", {
      prompt: prompt,
    })
      .then((response) => console.log(response))
      .catch((error) => console.error(error));
    setPrompt("");
  };

  const handleEnterSubmit = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter") {
      handleSpeakOllama();
    }
  };

  return (
    <div className="flex justify-between items-center px-4 py-2">
      <div className="flex justify-center items-center w-full gap-1">
        <Input
          type="prompt"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          onKeyDown={handleEnterSubmit}
        />
      </div>
      <Button size="icon" onClick={handleSpeakOllama}>
        <Image src={arrow} alt="Arrow" />
      </Button>
    </div>
  );
}
