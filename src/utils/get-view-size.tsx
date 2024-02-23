import { useEffect } from "react";
import useNexus from "@/store";

export default function GetViewSize() {
  const setViewHeight = useNexus((state) => state.setViewHeight);
  const setViewWidth = useNexus((state) => state.setViewWidth);

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
