import { Outlet, useNavigate, useSearch } from "@tanstack/react-router";
import { Search } from "./search/Search";
import { ProjectPanel } from "./project/ProjectPanel";
import { useProjects } from "@/hooks/api/projects/useProjects";
import { useSearchbar } from "../contexts/SearchbarContext";
import SearchResultsBox from "./search/SearchResultsBox";
import type Project from "@/domain/project/entity";
import { SearchBar } from "./search/SearchBar";
import { useRef, useState } from "react";

export const OverlayPanels = () => {
  const { projects } = useSearch({ from: "/_app/" });
  const { data } = useProjects();
  const { setIsOpen: setSearchOpen, isOpen: searchOpen } = useSearchbar();
  const loadedProjects = projects ? projects.split(",") : [];
  const projectsToShow = data
    ? data.filter((p) => loadedProjects.includes(p.slug))
    : [];
  const inputRef = useRef<HTMLInputElement>(null);
  const [searchText, setSearchText] = useState<string>("");
  const [filteredItems, setFilteredItems] = useState<Project[]>([]);
  const highlightedSearchIndexState = useState<number>(0);
  const navigate = useNavigate();
  const [selectedTab, setSelectedTab] = useState<"projects" | "search-sites">(
    "projects"
  );
  const handleSelect = (project: Project) => {
    setSearchText("");
    setSearchOpen(false);
    navigate({ to: project.url });
  };

  if (data) {
    return (
      <div
        id="overlay-panels"
        className="flex flex-col absolute top-0 p-4  pointer-events-none w-full min-w-0 min-h-0 gap-2 bottom-0"
      >
        <div className="pointer-events-auto">
          <SearchBar
            highlightedSearchIndexState={highlightedSearchIndexState}
            searchText={searchText}
            handleSelect={handleSelect}
            inputRef={inputRef}
            setSelectedTab={setSelectedTab}
            setSearchText={setSearchText}
            filteredItems={filteredItems}
          />
        </div>

        {searchOpen && (
          <SearchResultsBox
            selectedTab={selectedTab}
            setSelectedTab={setSelectedTab}
            highlightedSearchIndexState={highlightedSearchIndexState}
            searchText={searchText}
            filterSate={[filteredItems, setFilteredItems]}
            handleSelect={handleSelect}
            inputRef={inputRef}
          />
        )}

        {projectsToShow.length > 0 && (
          <div className="flex flex-col shadow-lg rounded-box gap-2 max-w-[600px] pointer-events-auto overflow-y-auto [scrollbar-gutter:stable]">
            {<ProjectPanels projects={projectsToShow} />}
          </div>
        )}
      </div>
    );
  }
  return null;
};

const ProjectPanels = ({ projects }: { projects: Project[] }) => {
  return projects.map((p) => <ProjectPanel key={p.slug} project={p} />);
};
