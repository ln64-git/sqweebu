"use client";
import useNexus from "@/store";
import { Burger } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";

export default function BurgerButton() {
  const [opened, { toggle }] = useDisclosure();
  const setSidebar = useNexus((state) => state.setSidebar);

  const handleClick = () => {
    setSidebar(!opened);
    toggle();
  };

  return (
    <div className="flex opacity-100 fixed top-0 left-0 pl-4 pt-1.5 items-center">
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
