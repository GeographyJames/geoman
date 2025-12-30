import { useRef, useState } from "react";
import SearchResultsBox from "./SearchResultsBox";
import Project from "@/domain/project/entity";
import { useNavigate } from "@tanstack/react-router";

import { useSearchbar } from "../../contexts/SearchbarContext";
import { SearchBar } from "./SearchBar";

export function Search() {
  const { setIsOpen: setSearchOpen, isOpen: searchOpen } = useSearchbar();
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

  return (
    <div
      id="search-bar"
      className="flex flex-col min-h-0 w-full sm:max-w-[28rem] pointer-events-auto"
    >
      <div className="bg-base-100 shadow-lg rounded-full">
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
        className={`grid transition-all duration-200 ease-out ${
          searchOpen
            ? "grid-rows-[1fr] opacity-100 [content-visibility:visible]"
            : "grid-rows-[0fr] opacity-0 [content-visibility:hidden]"
        }`}
      >
        <div className="overflow-hidden">
          <div className="bg-base-100 shadow-lg rounded-box pt-2 mt-2">
            <SearchResultsBox
              selectedTab={selectedTab}
              setSelectedTab={setSelectedTab}
              highlightedSearchIndexState={highlightedSearchIndexState}
              searchText={searchText}
              filterSate={[filteredItems, setFilteredItems]}
              handleSelect={handleSelect}
              inputRef={inputRef}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
