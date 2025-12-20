// import { useOverviewSidebarContext } from "@/providers/OverviewSidebarProvider";
import { useOverviewSidebar } from "@/features/app/contexts/SidebarContext";
import type { ReactNode } from "react";

import { MdOutlineKeyboardArrowLeft } from "react-icons/md";

interface Props {
  children: ReactNode;

  // showArchivedToggle?: () => JSX.Element;
  // newDataType?: NewDataType;
  // showArchivedContext?: React.Context<ShowArchivedType>;
}

export default function OverviewSidebar({ children }: Props) {
  const sidebar = useOverviewSidebar();

  return (
    <div
      id="overview_sidebar"
      className={`fixed top-0 left-0 h-full transition-transform sm:w-96 w-full pt-6 z-50 bg-white overflow-y-auto shadow-xl ${
        sidebar.isOpen ? "translate-x-0" : "-translate-x-full"
      }`}
    >
      <button
        onClick={() => sidebar.toggleSidebar()}
        className="absolute right-4 top-4 btn btn-sm btn-circle btn-ghost"
      >
        <MdOutlineKeyboardArrowLeft size={20} />
      </button>
      {children}
    </div>
  );
}
