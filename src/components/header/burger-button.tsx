"use client";
import useNexus from "@/store";
import { Burger } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";

export default function BurgerButton() {
  const [opened, { toggle }] = useDisclosure();
  const setSidebar = useNexus((state) => state.setSidebar);
  const darkMode = useNexus((state) => state.darkMode);

  const handleClick = () => {
    setSidebar(!opened);
    toggle();
  };

  return (
    <div className="flex items-center pl-[2.6px]">
      <Burger
        opened={opened}
        onClick={handleClick}
        aria-label="Toggle navigation"
        size={"sm"}
        color="dark"
        className="mr-4"
      />
    </div>
  );
}
