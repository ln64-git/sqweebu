import { useDisplayStore } from "@/store/display-store";
import { useEffect } from "react";

export default function GetViewSize() {
  const setViewHeight = useDisplayStore((state) => state.setViewHeight);
  const setViewWidth = useDisplayStore((state) => state.setViewWidth);

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
