import { useEffect } from "react";

import Project from "@/domain/project/entity";
import SidebarList from "../projectsList/ProjectList";
import { useProjects } from "@/hooks/api/useProjects";

interface Props {
  searchText: string;
  selectedTab: "projects" | "search-sites";
  setSelectedTab: React.Dispatch<
    React.SetStateAction<"projects" | "search-sites">
  >;

  filterSate: [Project[], React.Dispatch<React.SetStateAction<Project[]>>];
  highlightedSearchIndexState: [
    number,
    React.Dispatch<React.SetStateAction<number>>,
  ];
  handleSelect: (item: Project) => void;
  inputRef: React.RefObject<HTMLInputElement | null>;
}

export default function SearchResultsBox({
  setSelectedTab,
  selectedTab,
  searchText,
  filterSate: [filteredItems, setFilteredItems],
}: Props) {
  const { data: projects } = useProjects();
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

  return (
    <div className="flex flex-col min-h-0">
      <div className="flex  justify-between border-b border-base-300">
        <button
          className={`tab ${selectedTab === "projects" && "border-b-2 border-black tab-active"}`}
          onClick={() => setSelectedTab("projects")}
        >
          Projects
        </button>

        <button
          className={`tab ${selectedTab === "search-sites" && "border-b-2 border-black tab-active"}`}
          onClick={() => setSelectedTab("search-sites")}
        >
          Strategic Search Areas
        </button>
      </div>

      <div className=" flex flex-col min-h-0 ">
        {selectedTab === "projects" && (
          <div className=" min-h-0">
            <SidebarList items={filteredItems} />
          </div>
        )}

        {selectedTab === "search-sites" && (
          <div className="flex flex-col min-h-0">Tab content 2</div>
        )}
      </div>
    </div>
  );
}
