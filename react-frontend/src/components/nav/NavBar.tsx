import { useOverviewSidebar } from "@/contexts/SidebarContext";
import { Link } from "@tanstack/react-router";
import { HiDocumentText } from "react-icons/hi";
import { MdAdminPanelSettings } from "react-icons/md";
import { HiLocationMarker } from "react-icons/hi";

export default function NavBar() {
  const sidebar = useOverviewSidebar();
  return (
    <>
      <Link
        to="/projects"
        className="hover:bg-gray-200 justify-center place-items-center flex shadow-lg bg-white h-8 px-4 rounded-full font-semibold gap-2"
        activeProps={{
          className: "bg-gray-100",
        }}
        onClick={sidebar.toggleSidebar}
      >
        <HiLocationMarker size={18} />
        Projects
      </Link>
      <Link
        to="/docs"
        className="hover:bg-gray-200 place-items-center justify-center flex shadow-lg bg-white h-8 px-4 rounded-full font-semibold gap-2"
        activeProps={{
          className: "bg-gray-100",
        }}
      >
        <HiDocumentText size={18} />
        Docs
      </Link>
      <Link
        to="/admin"
        className="hover:bg-gray-200 place-items-center justify-center flex shadow-lg bg-white h-8 px-4 rounded-full font-semibold gap-2"
        activeProps={{
          className: "bg-gray-100",
        }}
      >
        <MdAdminPanelSettings size={18} />
        Admin
      </Link>
    </>
  );
}
