"use client";
import { Inter } from "next/font/google";
import "../config/globals.css";
import "@mantine/core/styles.css";
import SideBar from "@/components/sidebar/sidebar";
import Header from "@/components/header/header";
import ChatFooter from "@/components/footer";
import { LayoutProvider } from "@/components/utils/layout-provider";
const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <LayoutProvider>
          <Header />
          <div className="flex h-full fixed left-0 right-0  bottom-0 ">
            <SideBar />
            <div className=" h-full flex flex-col  w-full justify-between   mx-auto">
              <div className="overflow-y-auto flex-1">{children}</div>
              <ChatFooter />
            </div>
          </div>
        </LayoutProvider>
      </body>
    </html>
  );
}
