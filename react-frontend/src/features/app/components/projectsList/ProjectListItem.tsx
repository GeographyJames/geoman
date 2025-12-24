import { FaLocationDot } from "react-icons/fa6";
import { GiWindTurbine } from "react-icons/gi";

import Project from "@/domain/project/entity";

import UserInitials from "../UserInitials";
import { Link } from "@tanstack/react-router";
import { useSearchbar } from "@/features/app/contexts/SearchbarContext";
import { VisibilityConfig } from "@/domain/types";
import { ActionsDropdown } from "@/components/ActionsDropdown";
import { ToggleArchivedStatus } from "@/components/ToggleArchivedStatus";

export default function ProjectListItem({ item }: { item: Project }) {
  const { setIsOpen: setSearchOpen } = useSearchbar();
  const handleClick = () => {
    setSearchOpen(false);
  };

  return (
    <li key={item.id}>
      <div className="flex">
        <Link
          to={item.url}
          onClick={handleClick}
          className="flex justify-start flex-1"
        >
          <div
            className={`flex gap-2 ${
              item.status === "ARCHIVED" && "text-gray-400"
            }`}
          >
            <span className="w-6 flex items-center font-bold">{item.id}</span>
            {item.name}
          </div>

          <div className="flex ml-auto">
            {item instanceof Project && <ProjectIcons project={item} />}
          </div>
        </Link>
        <ActionsDropdown id={item.id}>
          <li>
            <ToggleArchivedStatus
              archived={item.archived}
              setArchived={() => {
                console.log("archive");
              }}
            />
          </li>
        </ActionsDropdown>
      </div>
    </li>
  );
}

function ProjectIcons({ project }: { project: Project }) {
  const vis = VisibilityConfig[project.visibility];
  const VisibilityIcon = vis.icon;
  return (
    <>
      <div className="flex w-6 items-center items-center justify-center">
        {project.hasWind && project.primaryLayoutTurbineCount && (
          <div
            className="tooltip tooltip-left"
            data-tip={`${project.primaryLayoutTurbineCount} turbines`}
          >
            <GiWindTurbine size={20} />
          </div>
        )}
      </div>
      <div className="flex w-6  items-center justify-center">
        {project.centroid && <FaLocationDot size={18} />}
      </div>
      <div className="flex w-6 items-center justify-center">
        <div
          className="tooltip tooltip-left"
          data-tip={`${vis.label}: ${vis.description}`}
        >
          <VisibilityIcon size={20} />
        </div>
      </div>
      <div className="flex w-8 itemc-center justify-center">
        <UserInitials
          message={`project owner: ${project.ownerFirstName} ${project.ownerLastName}`}
          firstName={project.ownerFirstName}
          lastName={project.ownerLastName}
        />
      </div>
    </>
  );
}
