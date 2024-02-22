"use client";
import useNexus from "@/store";
import { Modal } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import React, { useEffect } from "react";

export default function ModelModal() {
  const command = useNexus((state) => state.command);
  const [opened, { open, close }] = useDisclosure(false);

  useEffect(() => {
    if (command === "model") {
      open();
    }
  }, [command, open, close]);

  return (
    <div>
      <Modal opened={opened} onClose={close} title="Model">
        {/* Modal content */}
      </Modal>
    </div>
  );
}
