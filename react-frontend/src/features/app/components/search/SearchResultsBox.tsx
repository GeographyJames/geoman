import { useMemo } from "react";

import Project from "@/domain/project/entity";
import { useProjectsFilter } from "@/features/app/contexts/ProjectsFilterContext";
import ProjectsList from "../projectsList/ProjectList";

interface Props {
  searchText: string;
  selectedTab: "projects" | "search-sites";
  setSelectedTab: React.Dispatch<
    React.SetStateAction<"projects" | "search-sites">
  >;
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
  inputRef,
}: Props) {
  const { projects } = useProjectsFilter();

  const filteredItems = useMemo(() => {
    if (!projects) return [];
    if (searchText.trim() === "") return projects;
    return projects.filter((d) =>
      d.name.toLowerCase().startsWith(searchText.toLowerCase()),
    );
  }, [searchText, projects]);

  return (
    <div
      className="sm:max-w-[28rem] flex flex-col rounded-box pointer-events-auto min-h-56 bg-base-100 pt-2 shadow-lg  "
      onClick={() => inputRef.current?.blur()}
      onTouchStart={() => inputRef.current?.blur()}
    >
      <div className="mx-2 flex  justify-between border-b border-base-300">
        <button
          className={`tab text-base font-bold ${selectedTab === "projects" && "border-b-2 border-base-content tab-active"}`}
          onClick={() => setSelectedTab("projects")}
        >
          Projects
        </button>

        <button
          className={`tab text-base font-bold ${selectedTab === "search-sites" && "border-b-2 border-base-content tab-active"}`}
          onClick={() => setSelectedTab("search-sites")}
        >
          Strategic Search Areas
        </button>
      </div>

      <div className=" flex flex-col min-h-0 ">
        {selectedTab === "projects" && (
          <div className=" min-h-0">
            <ProjectsList projects={filteredItems} />
          </div>
        )}

        {selectedTab === "search-sites" && (
          <div className="flex flex-col min-h-0">Tab content 2</div>
        )}
      </div>
    </div>
  );
}
