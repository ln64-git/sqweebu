export default function Header() {
  return (
    <div className="fixed top-0 w-full bg-zinc-950 bg-opacity-60 z-10 flex justify-center items-center backdrop-filter backdrop-blur-md ">
      <div className="bg-zinc-950 bg-opacity-80 rounded-md m-1 backdrop-blur-sm">
        <input
          type="text"
          className="w-full caret-transparent rounded-md px-4  text-xs bg-transparent text-zinc-500 placeholder-zinc-400 text-center backdrop-blur-sm placeholder-opacity-50 outline-none focus:outline-none" // Removed selected effects and made the font smaller
          placeholder="Command"
        />
      </div>
    </div>
  );
}
