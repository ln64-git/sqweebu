import { Burger } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import Image from "next/image";
import PlaybackControls from "../playback/playback-controls";
import IconButton from "../playback/playback-icon-button";

export default function Header() {
  const [opened, { toggle }] = useDisclosure();

  return (
    <div className="fixed top-0 w-full bg-zinc-950 bg-opacity-60 z-10 flex justify-between items-center backdrop-filter backdrop-blur-xl px-4">
      <div className="flex items-center">
        <Burger
          opened={opened}
          onClick={toggle}
          aria-label="Toggle navigation"
          size={"sm"}
          color="dark"
          className="mr-4"
        />
      </div>
      <div className="flex justify-center flex-grow ">
        <div className="bg-zinc-950 w-full bg-opacity-80 rounded-md m-1 backdrop-blur-sm   ">
          <input
            type="text"
            className="w-full caret-transparent py-2 rounded-md px-4 text-xs bg-transparent text-zinc-500 placeholder-zinc-400 text-center backdrop-blur-lg placeholder-opacity-50 outline-none focus:outline-none"
            placeholder="Command"
          />
        </div>
      </div>
      <PlaybackControls />
    </div>
  );
}
