import { invoke } from "@tauri-apps/api";

export default function Home() {
  const handleClick = () => {
    invoke("speak_text", { text: "Hello from JavaScript!" })
      .then((response) => console.log(response))
      .catch((error) => console.error(error));
  };

  return (
    <main>
      <button onClick={handleClick}>Speak</button>
    </main>
  );
}
