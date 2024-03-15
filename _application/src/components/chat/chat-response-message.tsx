// import { useThemeColor } from "@/config/themes";

const ResponseMessage: React.FC<ChatEntry> = ({ body }) => {
  // const textPrimary = useThemeColor("textPrimary");

  return (
    <div style={{ color: "#FFFFFF" }} className="mx-2 my-2 mb-8 text-sm ">
      {body}
    </div>
  );
};

export default ResponseMessage;
