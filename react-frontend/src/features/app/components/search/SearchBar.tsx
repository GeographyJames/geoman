import SearchInput from "./SearchInput";
import { useProjects } from "@/hooks/api/useProjects";
import { useSidebar } from "@/features/app/contexts/SidebarContext";
import { useSearchbar } from "../../contexts/SearchbarContext";
import { ExpandButton, MenuButton } from "@/components/Buttons";
import { UserBadge } from "@/components/UserBadge";
import type Project from "@/domain/project/entity";

interface Props {
  searchText: string;
  setSearchText: React.Dispatch<React.SetStateAction<string>>;
  highlightedSearchIndexState: [
    number,
    React.Dispatch<React.SetStateAction<number>>,
  ];
  setSelectedTab: React.Dispatch<
    React.SetStateAction<"projects" | "search-sites">
  >;
  filteredItems: Project[];
  handleSelect: (item: Project) => void;
  inputRef: React.RefObject<HTMLInputElement | null>;
}

export const SearchBar = ({
  setSelectedTab,
  searchText,
  setSearchText,
  highlightedSearchIndexState,
  handleSelect,
  filteredItems,
  inputRef,
}: Props) => {
  const { isError, isPending } = useProjects();
  const { toggleSidebar } = useSidebar();
  const { toggleSearchbar, isOpen: searchOpen } = useSearchbar();
  return (
    <div className="flex pl-2 pr-3 gap-2 items-center">
      <MenuButton onClick={toggleSidebar}></MenuButton>
      <ExpandButton
        expanded={searchOpen}
        onClick={toggleSearchbar}
      ></ExpandButton>

      <SearchInput
        setSelectedTab={setSelectedTab}
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
      <UserBadge />
    </div>
  );
};
