import { useEffect } from "react";

import Project from "@/domain/project/entity";
import SidebarList from "../projectsList/ProjectList";
import { useProjects } from "@/hooks/api/projects/useProjects";

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
  inputRef,
}: Props) {
  const { data: projects } = useProjects();
  useEffect(() => {
    if (!projects) return;
    if (searchText.trim() !== "") {
      const filtered = projects.filter((d) =>
        d.name.toLowerCase().startsWith(searchText.toLowerCase())
      );

      setFilteredItems(filtered);
    } else {
      setFilteredItems(projects);
    }
  }, [searchText, projects, setFilteredItems]);

  return (
    <div
      className="sm:max-w-[28rem] flex flex-col rounded-box pointer-events-auto min-h-56 bg-orange-200 pt-2"
      onClick={() => inputRef.current?.blur()}
      onTouchStart={() => inputRef.current?.blur()}
    >
      <div className="mx-2 flex  justify-between border-b border-base-300">
        <button
          className={`tab ${selectedTab === "projects" && "border-b-2 border-base-content tab-active"}`}
          onClick={() => setSelectedTab("projects")}
        >
          Projects
        </button>

        <button
          className={`tab ${selectedTab === "search-sites" && "border-b-2 border-base-content tab-active"}`}
          onClick={() => setSelectedTab("search-sites")}
        >
          Strategic Search Areas
        </button>
      </div>

      <div className=" flex flex-col min-h-0 ">
        {selectedTab === "projects" && (
          <div className=" min-h-0">
            <SidebarList projects={filteredItems} />
          </div>
        )}

        {selectedTab === "search-sites" && (
          <div className="flex flex-col min-h-0">Tab content 2</div>
        )}
      </div>
    </div>
  );
}
