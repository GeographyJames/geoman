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
      className={`overflow-hidden gap-2 flex h-full flex-col min-h-0 w-[28rem] max-w-[calc(100vw-2rem)] bg-white shadow-lg pointer-events-auto ${searchOpen ? "rounded-box" : "rounded-full"}`}
    >
      <SearchBar
        highlightedSearchIndexState={highlightedSearchIndexState}
        searchText={searchText}
        handleSelect={handleSelect}
        inputRef={inputRef}
        setSelectedTab={setSelectedTab}
        setSearchText={setSearchText}
        filteredItems={filteredItems}
      />
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
    </div>
  );
}
