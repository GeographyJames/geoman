import { FaLocationDot } from "react-icons/fa6";
import { GiWindTurbine } from "react-icons/gi";

import Project from "@/domain/project/entity";

import { FaLock } from "react-icons/fa";

import UserInitials from "../UserInitials";
import { Link } from "@tanstack/react-router";
import { useSearchbar } from "@/features/app/contexts/SearchbarContext";

export default function SidebarListItem({ item }: { item: Project }) {
  const { setIsOpen: setSearchOpen } = useSearchbar();
  const handleClick = () => {
    setSearchOpen(false);
  };

  return (
    <li key={item.id}>
      <Link
        to={item.url}
        onClick={handleClick}
        className={` flex justify-start`}
      >
        <div
          className={`flex gap-2 ${
            item.status === "archived" && "text-gray-400"
          }`}
        >
          <span>{item.id}</span>
          {item.name}
        </div>

        <div className="flex gap-2 ml-auto items-center">
          {item instanceof Project && <ProjectIcons project={item} />}

          <>
            <UserInitials date={item.added} user={item.addedBy} />
            {/* <div>
                <ActionsDropdown entity={item} />
              </div> */}
          </>
        </div>
      </Link>
    </li>
  );
}

function ProjectIcons({ project }: { project: Project }) {
  return (
    <div className="flex gap-1 items-center w-16 justify-between">
      <div>{project.private && <FaLock size={18} />}</div>
      <div>
        {project.hasWind && project.primaryLayoutTurbineCount && (
          // <div
          //   className="sm:tooltip"
          //   data-tip={`primary layout turbine count: ${project.primaryLayoutTurbineCount}`}
          // >
          <GiWindTurbine size={22} />
          // </div>
        )}
      </div>
      <div>{project.centroid && <FaLocationDot size={20} />}</div>
    </div>
  );
}
