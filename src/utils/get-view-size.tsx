import { useEffect } from "react";
import useInterfaceStore from "@/interface-store";

export default function GetViewSize() {
  const setViewHeight = useInterfaceStore((state) => state.setViewHeight);
  const setViewWidth = useInterfaceStore((state) => state.setViewWidth);

  useEffect(() => {
    function handleResize() {
      setViewHeight(window.innerHeight);
      setViewWidth(window.innerWidth);
    }

    window.addEventListener("resize", handleResize);
    handleResize();

    return () => window.removeEventListener("resize", handleResize);
  }, [setViewHeight, setViewWidth]);

  return null;
}
