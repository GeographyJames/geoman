import { Outlet } from "@tanstack/react-router";
import { SearchPanel } from "./search/SearchPanel";

export const OverlayPanels = () => {
  return (
    <div
      id="overlay-panels"
      className="flex flex-col gap-2 absolute top-0 p-4 h-full bg-pink-500/30 pointer-events-none"
    >
      <div id="search-bar-container" className="bg-yellow-500/30 min-h-0">
        <SearchPanel />
      </div>

      <Outlet />
    </div>
  );
};
