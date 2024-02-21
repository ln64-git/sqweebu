import { Burger } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import React from "react";

export default function BurgerButton() {
  const [opened, { toggle }] = useDisclosure();

  return (
    <div className="flex items-center pl-[2.6px]">
      <Burger
        opened={opened}
        onClick={toggle}
        aria-label="Toggle navigation"
        size={"sm"}
        color="dark"
        className="mr-4"
      />
    </div>
  );
}
