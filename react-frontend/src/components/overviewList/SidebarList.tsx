import { useState } from "react";

import Project from "@/domain/project/entity";
import SortBy, { SORT_OPTIONS } from "./SortBy";
import SidebarListItem from "./SidebarListItem";
import { ShowArchivedProjectsToggle } from "../ShowArchivedToggle";
import { CreateButton } from "../Buttons";

interface Props {
  items: Project[];

  // showArchived: boolean;
  // setShowArchived: React.Dispatch<React.SetStateAction<boolean>>;
}

export default function SidebarList({ items }: Props) {
  const [sortBy, setSortBy] = useState(SORT_OPTIONS.NAME_ASCENDING);
  const sortedData = items.slice().sort((a, b) => {
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
    <div>
      <div className="px-4 pt-2 flex flex-col gap-2">
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
          <ShowArchivedProjectsToggle />

          <span className="flex ml-auto text-xs text-gray-500">{`showing ${sortedData.length}`}</span>
        </div>
      </div>
      <ul className="menu  w-full">
        {sortedData.map((item) => (
          <SidebarListItem key={item.id} item={item} />
        ))}
      </ul>
    </div>
  );
}
