import { memo } from "react";
import Project from "@/domain/project/entity";

import { Link, useSearch } from "@tanstack/react-router";
import { useSearchbarSetOpen } from "@/features/app/contexts/SearchbarContext";
import { useSetHoveredProject } from "@/features/app/contexts/ProjectsFilterContext";

import { ProjectIcons } from "../project/ProjectIcons";
import { ProjectActionsDropdown } from "../project/ProjectActionsDropdown";

function ProjectListItem({ item }: { item: Project }) {
  const setSearchOpen = useSearchbarSetOpen();
  const setHoveredProjectId = useSetHoveredProject();

  const { projects } = useSearch({ from: "/_app/" });

  const handleClick = () => {
    setSearchOpen(false);
  };

  const currentProjects = projects;
  const projectsArray = currentProjects ? currentProjects.split(",") : [];
  const newProjects = projectsArray.includes(item.slug)
    ? projectsArray.join(",")
    : [...projectsArray, item.slug].join(",");

  return (
    <li
      key={item.id}
      onMouseEnter={() => setHoveredProjectId(item.id)}
      onMouseLeave={() => setHoveredProjectId(null)}
    >
      <div className="flex p-0 pr-2">
        <Link
          from={"/"}
          search={{ projects: newProjects }}
          onClick={handleClick}
          className="flex justify-start flex-1 py-2 pl-2 items-center"
        >
          <div
            className={`flex gap-2 min-w-0 ${
              item.status === "ARCHIVED" && "text-base-content/50"
            }`}
          >
            <span className="w-12 flex items-center flex-shrink-0">
              {item.id}
            </span>
            <span>
              <span className="break-all">{item.name}</span>{" "}
              {item.status === "ARCHIVED" && (
                <span className="text-xs">(archived)</span>
              )}
            </span>
          </div>

          <div className="flex ml-auto">
            {item instanceof Project && <ProjectIcons project={item} />}
          </div>
        </Link>
        <ProjectActionsDropdown item={item} id={`li-p-${item.id}`} />
      </div>
    </li>
  );
}

export default memo(ProjectListItem);
