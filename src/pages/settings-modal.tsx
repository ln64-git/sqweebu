"use client";
import useNexus from "@/store";
import { Modal } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import React, { useEffect } from "react";

export default function SettingsModal() {
  const command = useNexus((state) => state.command);
  const [opened, { open, close }] = useDisclosure(false);

  useEffect(() => {
    if (command === "settings") {
      open();
    }
  }, [command, open, close]);

  return (
    <div className="">
      <Modal opened={opened} onClose={close} title="Settings">
        {/* Modal content */}awd
      </Modal>
    </div>
  );
}
