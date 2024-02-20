import React from "react";

export default function ChatMessage() {
  return (
    <div>
      <div className="flex items-start gap-2.5">
        <div className="flex flex-col gap-1 w-full max-w-[320px]">
          <div className="flex items-center space-x-2 rtl:space-x-reverse"></div>
          <div className="flex flex-col leading-1.5 p-4 rounded-e-xl rounded-es-xl dark:bg-zinc-800">
            <p className="text-sm font-normal X">
              That's awesome. I think our users will really appreciate the
              improvements.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
