import { Outlet } from "@tanstack/react-router";
import { Search } from "./search/Search";

export const OverlayPanels = () => {
  return (
    <div
      id="overlay-panels"
      className="flex flex-col gap-2 absolute top-0 p-4 h-full  pointer-events-none w-full min-w-0"
    >
      <Search />

      <Outlet />
    </div>
  );
};
