"use client";

import { useState } from "react";

const HomePage = () => {
  const [responseData, setResponseData] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      const response = await fetch("http://localhost:3025/");
      if (!response.ok) {
        throw new Error("Failed to fetch data");
      }
      const data = await response.text();
      console.log(data);
      setResponseData(data);
    } catch (error: any) {
      console.error("Error fetching data:", error.message);
    }
  };

  return (
    <div className="flex h-full mt-10 max-w-[580px] mx-auto ">
      <div className="flex-1 px-4 text-zinc-400 mt-1.5 gap-2">
        <button className="bg-indigo-950 rounded-md p-2" onClick={fetchData}>
          Fetch Data
        </button>
        {responseData && (
          <div className="mt-4">
            <p>Server Response:</p>
            <pre>{responseData}</pre>
          </div>
        )}
      </div>
    </div>
  );
};

export default HomePage;
