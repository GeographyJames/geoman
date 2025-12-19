import { useRef, useState } from "react";
import { MdChevronRight } from "react-icons/md";
import SearchResultsBox from "./SearchResultsBox";
import SearchInput from "./SearchInput";
import Project from "@/domain/project/entity";
import { useNavigate } from "@tanstack/react-router";
import { useProjects } from "@/hooks/api/useProjects";
import { IoMenu } from "react-icons/io5";
import { SignedIn, UserButton } from "@clerk/clerk-react";

interface Props {
  setSidebarOpen: React.Dispatch<React.SetStateAction<boolean>>;
  searchOpen: boolean;
  setSearchOpen: React.Dispatch<React.SetStateAction<boolean>>;
}

export function SearchBar({
  setSidebarOpen,
  searchOpen,
  setSearchOpen,
}: Props) {
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
      id="search-bar"
      className={`w-[28rem] max-w-[calc(100vw-2rem)] overflow-hidden bg-white ${searchOpen ? "rounded-box shadow-lg" : "rounded-full"}`}
    >
      <div
        className={`box-border flex bg-white px-4 items-center relative z-10 rounded-full ${!searchOpen && "shadow-lg"}`}
      >
        <button
          type="button"
          onClick={() => setSidebarOpen((prev) => !prev)}
          className="btn btn-ghost btn-circle btn-sm"
        >
          <IoMenu size={24} />
        </button>
        <button
          type="button"
          onClick={() => setSearchOpen(!searchOpen)}
          className="btn btn-ghost btn-circle btn-sm"
        >
          <MdChevronRight
            size={24}
            className={`transition-transform ${searchOpen ? "rotate-90" : ""}`}
          />
        </button>

        <SearchInput
          setSelectedTab={setSelectedTab}
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
        {__RUN_ENVIRONMENT__ === "demo" ? (
          <div
            className="tooltip tooltip-left"
            data-tip="User authentication disabled in demo mode"
          >
            <div className="badge badge-warning">Demo</div>
          </div>
        ) : (
          <SignedIn>
            <UserButton />
          </SignedIn>
        )}
      </div>
      {searchOpen && (
        <div className="tabs tabs-border justify-between" id="tab">
          <input
            type="radio"
            name="my_tabs_2"
            className="tab"
            aria-label="Projects"
            checked={selectedTab === 0}
            onChange={() => setSelectedTab(0)}
          />
          <div className="tab-content rounded-none border-b-0 border-base-200 p-0">
            {projects && (
              <SearchResultsBox
                setSearchOpen={setSearchOpen}
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
            onChange={() => setSelectedTab(1)}
          />
          <div className="tab-content   rounded-none border-b-0 border-base-200 p-0">
            Tab content 2
          </div>
        </div>
      )}
    </div>
  );
}
