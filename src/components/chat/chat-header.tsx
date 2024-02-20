import React from "react";

export default function ChatHeader() {
  return (
    <div className="m-2 ">
      <textarea
        style={{
          resize: "none",
          overflow: "hidden",
          outline: "none",
        }}
        className="block p-2.5 w-full text-sm rounded-lg bg-zinc-950"
        rows={1}
      ></textarea>
    </div>
  );
}
