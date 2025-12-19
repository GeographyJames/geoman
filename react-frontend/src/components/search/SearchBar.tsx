import { useRef, useState } from "react";
import { MdChevronRight } from "react-icons/md";
import SearchResultsBox from "./SearchResultsBox";
import SearchInput from "./SearchInput";
import Project from "@/domain/project/entity";
import { useNavigate } from "@tanstack/react-router";
import { useProjects } from "@/hooks/api/useProjects";
import { IoMenu } from "react-icons/io5";

export function SearchBar() {
  const [searchOpen, setSearchOpen] = useState<boolean>(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const [searchText, setSearchText] = useState<string>("");
  const [filteredItems, setFilteredItems] = useState<Project[]>([]);
  const highlightedSearchIndexState = useState<number>(0);
  const navigate = useNavigate();
  const { data: projects, isError, isPending } = useProjects();
  const [selectedTab, setSelectedTab] = useState<number>(0);

  const handleSelect = (project: Project) => {
    setSearchText("");
    setSearchOpen(false);
    navigate({ to: project.url });
  };

  return (
    <div
      className={`w-full min-w-96 sm:w-96 bg-white ${searchOpen ? "rounded-xl shadow-lg" : "rounded-full"}`}
    >
      <div
        className={`box-border flex bg-white px-4 items-center relative z-10 rounded-full ${!searchOpen && "shadow-lg"}`}
      >
        <button
          type="button"
          onClick={() => setSearchOpen(!searchOpen)}
          className="p-2 cursor-pointer"
        >
          <IoMenu size={24} />
        </button>
        <button
          type="button"
          onClick={() => setSearchOpen(!searchOpen)}
          className="p-2 cursor-pointer"
        >
          <MdChevronRight
            size={24}
            className={`transition-transform ${searchOpen ? "rotate-90" : ""}`}
          />
        </button>

        <SearchInput
          searchResultsOpenState={[searchOpen, setSearchOpen]}
          searchTextState={[searchText, setSearchText]}
          highlightedSearchIndexState={highlightedSearchIndexState}
          handleSelect={handleSelect}
          filteredItems={filteredItems}
          inputRef={inputRef}
          placeholderText={
            isError
              ? "error loading projects"
              : isPending
                ? "loading projects..."
                : "search projects"
          }
        />
      </div>
      {searchOpen && (
        <div className="tabs tabs-border justify-between" id="tab">
          <input
            type="radio"
            name="my_tabs_2"
            className="tab"
            aria-label="Projects"
            checked={selectedTab === 0}
            onClick={() => setSelectedTab(0)}
          />
          <div className="tab-content rounded-none border-b-0 border-base-200 p-0">
            {projects && (
              <SearchResultsBox
                projects={projects}
                highlightedSearchIndexState={highlightedSearchIndexState}
                searchText={searchText}
                filterSate={[filteredItems, setFilteredItems]}
                handleSelect={handleSelect}
                inputRef={inputRef}
              />
            )}
          </div>
          <input
            type="radio"
            name="my_tabs_2"
            className="tab"
            aria-label="Strategic Search Areas"
            checked={selectedTab === 1}
            onClick={() => setSelectedTab(1)}
          />
          <div className="tab-content   rounded-none border-b-0 border-base-200 p-0">
            Tab content 2
          </div>
        </div>
      )}
    </div>
  );
}
