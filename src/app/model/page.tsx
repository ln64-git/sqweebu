"use client";
import { Table } from "@mantine/core";
import React from "react";

const elements = [
  { position: 1, name: "Hydrogen", symbol: "H", mass: 1.008 },
  { position: 2, name: "Helium", symbol: "He", mass: 4.0026 },
  { position: 3, name: "Lithium", symbol: "Li", mass: 6.94 },
  // Add more mock data as needed
];

export default function ModelsPage() {
  const rows = elements.map((element) => (
    <Table.Tr key={element.name}>
      <Table.Td>{element.position}</Table.Td>
      <Table.Td>{element.name}</Table.Td>
      <Table.Td>{element.symbol}</Table.Td>
      <Table.Td>{element.mass}</Table.Td>
    </Table.Tr>
  ));
  return (
    <div className=" h-full p-4  flex flex-col max-w-[450px] mx-auto overflow-y-auto">
      <div className=" pb-2 pl-4">Select available ollama models ~</div>

      <div className=" flex h-full">
        <div>
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
        </div>
      </div>
      <div className=" ">
        <div className="flex justify-between w-2/3 mx-auto">
          <div>Download</div>
          <div>Select</div>
        </div>
      </div>
    </div>
  );
}
