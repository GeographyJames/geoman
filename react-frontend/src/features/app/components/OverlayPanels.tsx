import { SearchBar } from "@/features/app/components/search/SearchBar";
import { Outlet } from "@tanstack/react-router";

export const OverlayPanels = () => {
  return (
    <div
      id="overlay-panels"
      className="flex flex-col gap-2 absolute top-0 m-4 bg-pink-500/30 pointer-events-none"
    >
      <div id="search-bar" className="bg-yellow-500/30 pointer-events-auto">
        <SearchBar />
      </div>
      <div
        id="outlet"
        className="pointer-events-auto bg-white p-4 w-[600px] shadow-lg rounded-box relative"
      >
        <Outlet />
      </div>
    </div>
  );
};
