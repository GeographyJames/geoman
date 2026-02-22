import { useNavigate, useSearch } from "@tanstack/react-router";

import { ProjectPanel } from "./project/ProjectPanel";
import { useProjects } from "@/hooks/api/projects/useProjects";
import { useSearchbar } from "../contexts/SearchbarContext";
import { useProjectsFilter } from "../contexts/ProjectsFilterContext";
import { useFlash } from "../contexts/FlashMessageContext";
import SearchResultsBox from "./search/SearchResultsBox";
import { FlashAlert } from "@/components/FlashAlert";
import type Project from "@/domain/project/entity";
import { SearchBar } from "./search/SearchBar";
import { useRef, useMemo, useState } from "react";

export const OverlayPanels = () => {
  const { projects: projectsParam } = useSearch({ from: "/_app/" });
  const { data } = useProjects();
  const { projects } = useProjectsFilter();
  const { setIsOpen: setSearchOpen, isOpen: searchOpen } = useSearchbar();
  const { messages, removeFlash } = useFlash();
  const loadedProjects = projectsParam ? projectsParam.split(",") : [];
  const projectsToShow = data
    ? data.filter((p) => loadedProjects.includes(p.slug))
    : [];
  const inputRef = useRef<HTMLInputElement>(null);
  const [searchText, setSearchText] = useState<string>("");
  const highlightedSearchIndexState = useState<number>(0);
  const navigate = useNavigate();
  const [selectedTab, setSelectedTab] = useState<"projects" | "search-sites">(
    "projects",
  );

  const filteredItems = useMemo(() => {
    if (!projects) return [];
    if (searchText.trim() === "") return projects;
    return projects.filter((d) =>
      d.name.toLowerCase().startsWith(searchText.toLowerCase()),
    );
  }, [searchText, projects]);

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
        {messages.length > 0 && (
          <div className="flex flex-col gap-2 pointer-events-auto">
            {messages.map((m) => (
              <FlashAlert
                key={m.id}
                message={m.message}
                type={m.type}
                onClose={() => removeFlash(m.id)}
              />
            ))}
          </div>
        )}
        <div className="pointer-events-none">
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

        <div
          className={`grid transition-all duration-300 ease-in-out ${
            searchOpen
              ? "grid-rows-[1fr] opacity-100 min-h-56"
              : "grid-rows-[0fr] opacity-0 min-h-0 -mb-2"
          }`}
          style={{ flex: "0 1 auto" }}
        >
          <div className="overflow-hidden min-h-0 flex flex-col">
            <SearchResultsBox
              selectedTab={selectedTab}
              setSelectedTab={setSelectedTab}
              highlightedSearchIndexState={highlightedSearchIndexState}
              searchText={searchText}
              handleSelect={handleSelect}
              inputRef={inputRef}
            />
          </div>
        </div>

        {projectsToShow.length > 0 && (
          <div className="flex flex-col shadow-lg rounded-box gap-2 max-w-[600px] pointer-events-auto min-h-12 overflow-y-auto ">
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
