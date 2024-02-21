"use client";
import useNexus from "@/store";
import React, { ReactNode } from "react";

interface ThemeWrapperProps {
  children: ReactNode;
}

const ThemeWrapper: React.FC<ThemeWrapperProps> = ({ children }) => {
  const darkMode = useNexus((state) => state.darkMode);

  return <div className="flex flex-col h-screen bg-zinc-900">{children}</div>;
};

export default ThemeWrapper;
