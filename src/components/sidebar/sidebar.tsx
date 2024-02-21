import Image from "next/image";
import React from "react";

import settings from "../../../public/sidebar/settings.svg";

export default function SideBar() {
  return (
    <div className="flex flex-col items-center">
      <Image src={settings} alt="settings" width={30} />
      <Image src={settings} alt="settings" width={30} />
      <Image src={settings} alt="settings" width={30} />
      <Image src={settings} alt="settings" width={30} />
    </div>
  );
}
