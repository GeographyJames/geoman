import { FaLocationDot } from "react-icons/fa6";
import { GiWindTurbine } from "react-icons/gi";

import Project from "@/domain/project/entity";

import { FaLock } from "react-icons/fa";

import UserInitials from "../UserInitials";
import { Link } from "@tanstack/react-router";
import { useSearchbar } from "@/features/app/contexts/SearchbarContext";

export default function ProjectListItem({ item }: { item: Project }) {
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
          <span className="w-6 flex items-center font-bold">{item.id}</span>
          {item.name}
        </div>

        <div className="flex ml-auto">
          {item instanceof Project && <ProjectIcons project={item} />}

          <>
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
    <>
      <div className="flex w-6 items-center justify-center">
        {project.private && <FaLock size={16} />}
      </div>
      <div className="flex w-6 items-center items-center justify-center">
        {project.hasWind && project.primaryLayoutTurbineCount && (
          <div
            className="tooltip tooltip-left"
            data-tip={`${project.primaryLayoutTurbineCount} turbines`}
          >
            <div>
              <GiWindTurbine size={20} />
            </div>
          </div>
        )}
      </div>
      <div className="flex w-6  items-center justify-center">
        {project.centroid && <FaLocationDot size={20} />}
      </div>
      <div className="flex w-8 itemc-center justify-center">
        <UserInitials date={project.added} user={project.addedBy} />
      </div>
    </>
  );
}
