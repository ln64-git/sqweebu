import React from "react";
import { Button } from "./ui/button";
import { invoke } from "@tauri-apps/api";

export default function Welcome() {
  const handleSpeakText = () => {
    invoke("speak_text_from_frontend", { text: "Hello from the Front-end!" })
      .then((response) => console.log(response))
      .catch((error) => console.error(error));
  };
  const handleSpeakOllama = () => {
    invoke("speak_ollama_from_frontend", {
      prompt: "What is your favorite book?",
    })
      .then((response) => console.log(response))
      .catch((error) => console.error(error));
  };
  const handleSpeakClipboard = () => {
    invoke("speak_clipboard_from_frontend", {
      text: "Hello from the Front-end!",
    })
      .then((response) => console.log(response))
      .catch((error) => console.error(error));
  };

  return (
    <div className="flex-grow flex flex-col justify-center items-center">
      <h1 className="text-zinc-400 pb-4 text-2xl">Sqweebu</h1>
      <div className="w-3/4 flex flex-col gap-2 max-w-[200px]">
        <Button variant="home" onClick={handleSpeakText}>
          Speak Text
        </Button>
        <Button variant="home" onClick={handleSpeakOllama}>
          Speak Ollama
        </Button>
        <Button variant="home" onClick={handleSpeakClipboard}>
          Speak Clipboard
        </Button>
      </div>
    </div>
  );
}