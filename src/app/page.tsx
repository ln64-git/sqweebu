"use client";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api";

export default function Home() {
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
    <main>
      <div className="h-screen flex flex-col justify-center items-center">
        <h1 className="text-stone-400 pb-4 text-2xl">Navi Voice</h1>
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
    </main>
  );
}
