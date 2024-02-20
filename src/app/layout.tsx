import { Inter } from "next/font/google";
import "../config/globals.css";
const inter = Inter({ subsets: ["latin"] });
import theme from "../config/theme";
import type { Metadata } from "next";

import "@mantine/core/styles.css";

import { ColorSchemeScript, MantineProvider } from "@mantine/core";

export const metadata: Metadata = {
  title: "Navi AI Voice Assistant",
  description:
    "Interact with Artifical Intelligence in a natural conversational tone.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <MantineProvider theme={theme} defaultColorScheme="dark">
          {children}
        </MantineProvider>
      </body>
    </html>
  );
}
