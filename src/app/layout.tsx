import { Inter } from "next/font/google";
import "../config/globals.css";
const inter = Inter({ subsets: ["latin"] });
import { Inter as FontSans } from "next/font/google";
import type { Metadata } from "next";
import { cn } from "@/lib/utils";

export const metadata: Metadata = {
  title: "Sqweebu AI Voice Assistant",
  description:
    "Interact with Artifical Intelligence in a natural conversational tone.",
};

export const fontSans = FontSans({
  subsets: ["latin"],
  variable: "--font-sans",
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={cn(
          "min-h-screen bg-stone-950 font-sans antialiased" + inter.className
        )}
      >
        {children}
      </body>
    </html>
  );
}
