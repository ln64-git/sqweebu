import { invoke } from "@tauri-apps/api";

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
