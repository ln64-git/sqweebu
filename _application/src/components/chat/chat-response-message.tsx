import { useTheme } from "@/utils/theme-provider"; // Adjust the import path to where your ThemeProvider and useTheme are defined

const ResponseMessage: React.FC<ChatEntry> = ({ body }) => {
  const { theme } = useTheme();
  const textPrimary = theme.textPrimary;

  return (
    <div style={{ color: textPrimary }} className="mx-2 my-2 mb-8 text-sm ">
      {body}
    </div>
  );
};

export default ResponseMessage;
