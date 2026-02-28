import { memo, useMemo, useState } from "react";

import { CreateButton } from "../../../../components/Buttons";
import ProjectListItem from "./ProjectListItem";
import SortBy, { SORT_OPTIONS } from "./SortBy";
import ShowArchivedToggle from "../ShowArchivedToggle";
import { useProjectsFilter } from "@/features/app/contexts/ProjectsFilterContext";

import type Project from "@/domain/project/entity";

interface Props {
  projects?: Project[];
}

function ProjectsList({ projects: projectsProp }: Props = {}) {
  const {
    allProjects,
    projects: contextProjects,
    showArchivedProjects,
    setShowArchivedProjects,
  } = useProjectsFilter();
  const [sortBy, setSortBy] = useState(SORT_OPTIONS.NAME_ASCENDING);

  const sortedProjects = useMemo(
    () =>
      (projectsProp ?? contextProjects).slice().sort((a, b) => {
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
      }),
    [projectsProp, contextProjects, sortBy],
  );

  return (
    <div className="flex flex-col min-h-0 h-full pb-2 pt-4 gap-2">
      <div className="px-4 flex flex-col gap-2">
        <div className="flex justify-between gap-2">
          <div>
            <SortBy sortBy={sortBy} setSortBy={setSortBy} />
          </div>
          <div className=" flex flex-col gap-2 items-end">
            <CreateButton
              text="Create project"
              onClick={() => {
                const el = document.getElementById("create_project");
                if (el instanceof HTMLDialogElement) {
                  el.showModal();
                }
              }}
            />
            <div className="flex justify-end">
              <div className="flex flex-col items-end gap-y-1">
                <span className="text-xs text-base-content/70">{`showing ${sortedProjects.length}`}</span>
                <ShowArchivedToggle
                  showArchived={showArchivedProjects}
                  setShowArchived={setShowArchivedProjects}
                  archivedCount={allProjects.filter((p) => p.archived).length}
                />
              </div>
            </div>
          </div>
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

export default memo(ProjectsList);
