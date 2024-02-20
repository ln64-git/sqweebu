"use client";
import ChatFooter from "@/components/chat/chat-footer";
import ChatMain from "@/components/chat/chat-main";

export default function Home() {
  return (
    <div>
      <div className="fixed top-0 w-full bg-zinc-900 pt-3 z-10 "></div>
      <div className="bg-zinc-900 flex justify-center ">
        <div className="flex flex-col min-h-screen text-zinc-400 max-w-2xl">
          <ChatMain />
          <ChatFooter />
        </div>
      </div>
    </div>
  );
}
