import { ChatEntry } from "@/app/page";
import { useTheme } from "../utils/theme-provider";

const ResponseMessage: React.FC<ChatEntry> = ({ timestamp, content }) => {
  const { theme } = useTheme();
  const textPrimary = theme.textPrimary;

  return (
    <div style={{ color: textPrimary }} className="mx-2 text-sm ">
      {content}
    </div>
  );
};

export default ResponseMessage;
