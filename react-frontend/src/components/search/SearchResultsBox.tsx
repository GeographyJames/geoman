import { useEffect } from "react";

import Project from "@/domain/project/entity";
import SidebarList from "../overviewList/SidebarList";

interface Props {
  projects: Project[];
  searchText: string;

  filterSate: [Project[], React.Dispatch<React.SetStateAction<Project[]>>];
  highlightedSearchIndexState: [
    number,
    React.Dispatch<React.SetStateAction<number>>,
  ];
  handleSelect: (item: Project) => void;
  inputRef: React.RefObject<HTMLInputElement | null>;
}

export default function SearchResultsBox({
  projects,

  searchText,
  filterSate: [filteredItems, setFilteredItems],
}: Props) {
  useEffect(() => {
    if (!projects) return;
    if (searchText.trim() !== "") {
      const filtered = projects
        .filter((p) => p.searchAreaId === null)
        .filter((d) =>
          d.name.toLowerCase().startsWith(searchText.toLowerCase())
        );

      setFilteredItems(filtered);
    } else {
      const allProjects = projects.filter((p) => p.searchAreaId === null);
      setFilteredItems(allProjects);
    }
  }, [searchText, projects, setFilteredItems]);

  return <SidebarList items={filteredItems} />;
}
