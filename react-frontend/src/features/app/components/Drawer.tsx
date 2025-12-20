import BaseMap from "@/features/app/components/map/BaseMap";

import { useMapContext } from "@/features/app/contexts/MapRefContext";
import { Sidebar } from "./Sidebar";
import { OverlayPanels } from "./OverlayPanels";
import { useSidebar } from "@/features/app/contexts/SidebarContext";
import { useSearchbar } from "../contexts/SearchbarContext";

export const Drawer = () => {
  const sidebar = useSidebar();
  return (
    <div className="drawer h-full">
      <input
        id="my-drawer-1"
        type="checkbox"
        className="drawer-toggle"
        checked={sidebar.isOpen}
        onChange={sidebar.toggleSidebar}
      />
      <DrawerMain />
      <DrawerSide />
    </div>
  );
};

const DrawerMain = () => {
  const { setIsOpen: setSearchOpen } = useSearchbar();
  const { containerRef } = useMapContext();
  return (
    <div className="drawer-content h-full">
      <BaseMap
        containerRef={containerRef}
        onMouseDown={() => setSearchOpen(false)}
      />

      <OverlayPanels />
    </div>
  );
};

const DrawerSide = () => {
  return (
    <div className="drawer-side">
      <label
        htmlFor="my-drawer-1"
        aria-label="close sidebar"
        className="drawer-overlay"
      ></label>
      <Sidebar />
    </div>
  );
};
