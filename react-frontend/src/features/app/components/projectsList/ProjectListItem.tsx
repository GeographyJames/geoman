import Project from "@/domain/project/entity";

import { Link, useSearch } from "@tanstack/react-router";
import { useSearchbar } from "@/features/app/contexts/SearchbarContext";

import { ProjectIcons } from "../project/ProjectIcons";
import { ProjectActionsDropdown } from "../project/ProjectActionsDropdown";

export default function ProjectListItem({ item }: { item: Project }) {
  const { setIsOpen: setSearchOpen } = useSearchbar();

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
    <li key={item.id}>
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
            <span className="w-6 flex items-center font-bold flex-shrink-0">
              {item.id}
            </span>
            <span className="break-all min-w-0">{item.name}</span>
          </div>

          <div className="flex ml-auto">
            {item instanceof Project && <ProjectIcons project={item} />}
          </div>
        </Link>
        <ProjectActionsDropdown item={item} />
      </div>
    </li>
  );
}
