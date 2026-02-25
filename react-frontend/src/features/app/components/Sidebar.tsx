import { Link } from "@tanstack/react-router";
import { HiInformationCircle } from "react-icons/hi";
import { MdAdminPanelSettings } from "react-icons/md";

export const Sidebar = () => {
  return (
    <ul className="menu bg-base-200 min-h-full w-80 p-4">
      <li>
        <Link to="/admin" className="flex gap-2 items-center">
          <MdAdminPanelSettings size={18} />
          Admin
        </Link>
        <ul>
          <li>
            <Link to="/admin" search={{ tab: "teams" }}>
              Teams
            </Link>
          </li>
          <li>
            <Link to="/admin" search={{ tab: "collections" }}>
              Collections
            </Link>
          </li>
          <li>
            <Link to="/admin" search={{ tab: "api-keys" }}>
              API Keys
            </Link>
          </li>
        </ul>
      </li>
      <li>
        <Link to="/about" className="flex gap-2 items-center">
          <HiInformationCircle size={18} />
          About
        </Link>
      </li>
    </ul>
  );
};
