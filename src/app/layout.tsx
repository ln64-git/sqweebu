"use client";
import { useEffect } from "react";
import { Inter } from "next/font/google";
import "../config/globals.css";
import theme from "../config/mantine-theme";

import "@mantine/core/styles.css";

import { MantineProvider } from "@mantine/core";
import SideBar from "@/components/sidebar/sidebar";
import Header from "@/components/header/header";
import ChatFooter from "@/components/footer";
import { useThemeColor } from "@/config/themes";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const backgroundColor = useThemeColor("background");

  useEffect(() => {
    // Add top margin to the scrollbar
    const style = document.createElement("style");
    style.innerHTML = `
      ::-webkit-scrollbar {
        margin-top: 20px; /* Adjust this value to fit your header height */
      }
    `;
    document.head.appendChild(style);

    return () => {
      // Clean up the style tag when component unmounts
      document.head.removeChild(style);
    };
  }, []);

  return (
    <html lang="en">
      <body className={inter.className}>
        <MantineProvider theme={theme} defaultColorScheme="dark">
          <div style={{ backgroundColor }} className="flex flex-col h-screen">
            <div className="flex-1 flex flex-col overflow-y-auto ">
              <Header />
              <div className="flex flex-col items-center justify-center"></div>
              <div className="flex-1 w-full mx-auto overflow-y-auto">
                <div className="flex h-full">
                  <SideBar />
                  <div className="w-full h-full flex flex-col justify-between">
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
