"use client";
import useInterfaceStore from "@/interface-store";
import { Modal } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import React, { useEffect } from "react";

export default function SettingsModal() {
  const command = useInterfaceStore((state) => state.command);
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
