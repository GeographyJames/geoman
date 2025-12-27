import Project from "@/domain/project/entity";
import { IoClose, IoSearch } from "react-icons/io5";
import { useSearchbar } from "../../contexts/SearchbarContext";

interface Props {
  searchTextState: [string, React.Dispatch<React.SetStateAction<string>>];

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
  placeholderText: string;
}
export default function SearchInput({
  handleSelect,
  setSelectedTab,
  placeholderText,
  searchTextState: [searchText, setSearchText],
  highlightedSearchIndexState: [highlightedIndex, setHighlightedIndex],
  filteredItems,
  inputRef,
}: Props) {
  const { isOpen: searchOpen, setIsOpen: setSearchOpen } = useSearchbar();
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (!searchOpen) return;

    // Arrow down
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setHighlightedIndex((prev) =>
        prev < filteredItems.length - 1 ? prev + 1 : 0
      );
    }
    // Arrow up
    else if (e.key === "ArrowUp") {
      e.preventDefault();
      setHighlightedIndex((prev) =>
        prev > 0 ? prev - 1 : filteredItems.length - 1
      );
    }
    // Enter
    else if (
      e.key === "Enter" &&
      highlightedIndex >= 0 &&
      filteredItems.length > 0
    ) {
      e.preventDefault();
      handleSelect(filteredItems[highlightedIndex]);
    }
    // Escape
    else if (e.key === "Escape") {
      e.preventDefault();
      setSearchOpen(false);
    }
  };
  return (
    <label className="min-w-0 pl-0 my-2 input input-ghost border-none focus-within:outline-none  flex-grow  ">
      <input
        ref={inputRef}
        onChange={(e) => setSearchText(e.target.value)}
        value={searchText}
        onFocus={() => {
          setSearchOpen(true);
          setSelectedTab("projects");
        }}
        type="text"
        id="search"
        placeholder={placeholderText}
        name="search"
        onKeyDown={handleKeyDown}
      />

      {searchText && (
        <button
          className="btn btn-circle btn-ghost btn-xs"
          onClick={() => setSearchText("")}
          aria-label="clear search text"
        >
          <IoClose size={16} />
        </button>
      )}

      <div>
        <IoSearch />
      </div>
    </label>
  );
}
