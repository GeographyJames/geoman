interface Props {
  sortBy: string;
  setSortBy: (value: string) => void;
}

export default function SortBy({ sortBy, setSortBy }: Props) {
  return (
    <div className="flex items-center gap-2">
      <label htmlFor="sort-by" className="label-text whitespace-nowrap">
        Sort by
      </label>
      <select
        onChange={(e) => setSortBy(e.target.value)}
        id="sort-by"
        className="select select-bordered w-40 select-sm max-w-xs"
        value={sortBy}
      >
        <option value={SORT_OPTIONS.NAME_ASCENDING}>Name: A to Z</option>
        <option value={SORT_OPTIONS.CREATED}>Recently created</option>
        <option value={SORT_OPTIONS.ID_ASCENDING}>Id ascending</option>
        <option value={SORT_OPTIONS.ID_DESCENDING}>Id descending</option>
      </select>
    </div>
  );
}
export const SORT_OPTIONS = {
  NAME_ASCENDING: "name_ascending",
  CREATED: "created",
  ID_ASCENDING: "id_ascending",
  ID_DESCENDING: "id_descending",
};
