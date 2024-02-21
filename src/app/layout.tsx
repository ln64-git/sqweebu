"use client";
import { Inter } from "next/font/google";
import "../config/globals.css";
import theme from "../config/mantine-theme";

import "@mantine/core/styles.css";

import { MantineProvider } from "@mantine/core";
import SideBar from "@/components/sidebar/sidebar";
import Header from "@/components/header/header";
import ChatFooter from "@/components/footer";
import useNexus from "@/store";
import { useEffect, useState } from "react";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const lightTheme = useNexus((state) => state.lightTheme);
  const darkTheme = useNexus((state) => state.darkTheme);
  const darkMode = false;

  // Initialize the backgroundColor directly as an empty string
  const [backgroundColor, setBackgroundColor] = useState<string>("");

  useEffect(() => {
    const currentTheme = darkMode ? darkTheme : lightTheme;
    setBackgroundColor(currentTheme.background);
  }, [darkMode, lightTheme, darkTheme]);

  return (
    <html lang="en">
      <body className={inter.className}>
        <MantineProvider theme={theme} defaultColorScheme="dark">
          <div style={{ backgroundColor }} className="flex flex-col h-screen">
            <div className="flex-1 flex flex-col overflow-y-auto">
              <div className="flex items-center justify-center pb-10 ">
                <Header />
              </div>
              <div className="flex-1 w-full mx-auto overflow-y-auto">
                <div className="flex h-full ">
                  <SideBar />
                  <div className="w-full h-full flex flex-col justify-between ">
                    <div className="flex-1">{children}</div>
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
