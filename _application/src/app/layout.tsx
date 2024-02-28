"use client";
import { Inter } from "next/font/google";
import "../config/globals.css";
import theme from "../config/mantine-theme";
import "@mantine/core/styles.css";
import { MantineProvider } from "@mantine/core";
import SideBar from "@/components/sidebar/sidebar";
import Header from "@/components/header/header";
import ChatFooter from "@/components/footer";
import { useThemeColor } from "@/config/themes";
import SettingsModal from "@/pages/settings-modal";
import ChatModal from "@/pages/chat-modal ";
import ModelModal from "@/pages/model-modal";
import CommandModal from "@/pages/command-modal ";
import KeyboardModal from "@/pages/keyboard-modal ";
import VoiceModal from "@/pages/voice-modal ";
import GetViewSize from "@/utils/get-view-size";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const backgroundColor = useThemeColor("background");
  const textPrimary = useThemeColor("textPrimary");

  return (
    <html lang="en">
      <body className={inter.className}>
        <GetViewSize />
        <MantineProvider theme={theme} defaultColorScheme="dark">
          <SettingsModal />
          <ChatModal />
          <ModelModal />
          <VoiceModal />
          <KeyboardModal />
          <CommandModal />
          <div
            style={{ backgroundColor, color: textPrimary }}
            className="flex flex-col h-screen "
          >
            <div className="flex-1 flex flex-col items-center justify-center ">
              <Header />
              <div className="flex flex-col items-center justify-center"></div>
              <div className="flex-1 w-full mx-auto">
                <div className="flex h-full fixed left-0 right-0  bottom-0 ">
                  <SideBar />
                  <div className=" h-full flex flex-col  w-full justify-between   mx-auto">
                    <div className="overflow-y-auto flex-1">{children}</div>
                    <ChatFooter />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </MantineProvider>
      </body>
    </html>
  );
}
