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
    <div
      onClick={handleClick}
      className="sticky top-0 left-0 z-50 h-[40px]  justify-center w-[60px] pt-[4px] pl-[16.8px] items-center cursor-pointer"
    >
      <Burger
        opened={opened}
        aria-label="Toggle navigation"
        size={"sm"}
        color="dark"
        className="mr-4"
      />
    </div>
  );
}
