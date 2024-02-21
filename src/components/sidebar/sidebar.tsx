import Image from "next/image";
import React from "react";

import settings from "../../../public/sidebar/settings.svg";

export default function SideBar() {
  return (
    <div className="flex flex-col justify-between  h-full">
      <div className="flex flex-col items-center gap-2 pt-3">
        <Image src={settings} alt="settings" width={35} />
      </div>
      {/* <div className="flex flex-col items-center gap-2 pb-3">
        <Image src={settings} alt="settings" width={35} />
      </div> */}
    </div>
  );
}
