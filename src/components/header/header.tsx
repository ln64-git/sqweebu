import PlaybackControls from "./playback/playback-controls";
import CommandInput from "./command-input";
import BurgerButton from "./burger-button";

export default function Header() {
  return (
    <div className="fixed top-0 w-full bg-zinc-950 bg-opacity-60 z-10 flex justify-between items-center  px-4">
      <BurgerButton />
      <CommandInput />
      <PlaybackControls />
    </div>
  );
}
