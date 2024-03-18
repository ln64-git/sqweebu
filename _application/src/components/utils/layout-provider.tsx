"use client";
import { MantineProvider } from "@mantine/core";
import { ReactNode } from "react";
import { ThemeProvider } from "./theme-provider";
import mantineTheme from "../../config/mantine-theme";
import GetViewSize from "./get-view-size";

export const LayoutProvider = ({ children }: { children: ReactNode }) => {
  return (
    <MantineProvider theme={mantineTheme} defaultColorScheme="dark">
      <GetViewSize />
      <ThemeProvider>
        <div className="flex-1 w-full mx-auto">{children}</div>
      </ThemeProvider>
    </MantineProvider>
  );
};
