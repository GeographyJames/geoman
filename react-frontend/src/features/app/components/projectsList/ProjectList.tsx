import { useState } from "react";

import Project from "@/domain/project/entity";

import { CreateButton } from "../../../../components/Buttons";
import ProjectListItem from "./ProjectListItem";
import SortBy, { SORT_OPTIONS } from "./SortBy";
import ShowArchivedToggle from "../ShowArchivedToggle";

interface Props {
  projects: Project[];
}

export default function ProjectsList({ projects }: Props) {
  const [showArchived, setShowArchived] = useState<boolean>(false);
  const [sortBy, setSortBy] = useState(SORT_OPTIONS.NAME_ASCENDING);
  const sortedProjects = projects
    .filter((a) => showArchived || !a.archived)
    .slice()
    .sort((a, b) => {
      switch (sortBy) {
        case "name_ascending":
          return a.name.localeCompare(b.name);
        case "created":
          return b.added.getTime() - a.added.getTime();
        case "id_ascending":
          return a.id - b.id;
        case "id_descending":
          return b.id - a.id;
        default:
          return 0;
      }
    });

  return (
    <div className="flex flex-col min-h-0 h-full pb-2 pt-4 gap-2">
      <div className="px-4 flex flex-col gap-4">
        <div className="flex justify-between flex-wrap gap-2">
          <CreateButton
            text="project"
            onClick={() => {
              const el = document.getElementById("create_project");
              if (el instanceof HTMLDialogElement) {
                el.showModal();
              }
            }}
          />
          <SortBy sortBy={sortBy} setSortBy={setSortBy} />
        </div>

        <div className="flex justify-between">
          <ShowArchivedToggle
            showArchived={showArchived}
            setShowArchived={setShowArchived}
          />

          <span className="flex ml-auto text-xs text-gray-500">{`showing ${sortedProjects.length}`}</span>
        </div>
      </div>
      <div className="flex flex-col  overflow-y-auto scroll-gutter-stable scroll-panel py-1">
        <ul className="menu w-full  pb-0">
          {sortedProjects.map((item) => (
            <ProjectListItem key={item.id} item={item} />
          ))}
        </ul>
      </div>
    </div>
  );
}
