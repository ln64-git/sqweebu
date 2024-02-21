import { Inter } from "next/font/google";
import "../config/globals.css";
const inter = Inter({ subsets: ["latin"] });
import theme from "../config/theme";
import type { Metadata } from "next";

import "@mantine/core/styles.css";

import { ColorSchemeScript, MantineProvider } from "@mantine/core";
import ChatFooter from "@/components/chat/chat-footer";
import SideBar from "@/components/sidebar/sidebar";
import Header from "@/components/header/header";

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
          <div className="flex flex-col h-screen bg-zinc-900">
            <div className="flex items-center justify-center pb-10">
              <Header />
            </div>
            <div className="flex flex-1 overflow-y-auto ">
              <SideBar />
              <div className="flex-1 flex flex-col">
                <div className="flex-1 max-w-2xl mx-auto overflow-y-auto">
                  {children}
                </div>
                <div>
                  <ChatFooter />
                </div>
              </div>
            </div>
          </div>
        </MantineProvider>
      </body>
    </html>
  );
}
