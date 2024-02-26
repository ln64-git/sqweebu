"use client";
import useInterfaceStore from "@/interface-store";
import { Modal, Table } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import React, { useEffect } from "react";

export default function VoiceModal() {
  const elements = [
    { position: 1, name: "Hydrogen", symbol: "H", mass: 1.008 },
    { position: 2, name: "Helium", symbol: "He", mass: 4.0026 },
    { position: 3, name: "Lithium", symbol: "Li", mass: 6.94 },
  ];
  const rows = elements.map((element) => (
    <Table.Tr key={element.name}>
      <Table.Td>{element.position}</Table.Td>
      <Table.Td>{element.name}</Table.Td>
      <Table.Td>{element.symbol}</Table.Td>
      <Table.Td>{element.mass}</Table.Td>
    </Table.Tr>
  ));

  const command = useInterfaceStore((state) => state.command);
  const [opened, { open, close }] = useDisclosure(false);

  useEffect(() => {
    if (command === "voice") {
      open();
    }
  }, [command, open, close]);

  return (
    <div>
      <Modal opened={opened} onClose={close} title="Voice">
        <Table>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Element position</Table.Th>
              <Table.Th>Element name</Table.Th>
              <Table.Th>Symbol</Table.Th>
              <Table.Th>Atomic mass</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>{rows}</Table.Tbody>
        </Table>
      </Modal>
    </div>
  );
}
