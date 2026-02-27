export default function ShowArchivedToggle({
  showArchived,
  setShowArchived,
  archivedCount,
}: {
  showArchived: boolean;
  setShowArchived: (val: boolean) => void;
  archivedCount?: number;
}) {
  return (
    <div className="flex content-center ">
      <label className="label cursor-pointer p-0">
        <span className="label-text text-xs ">
          show archived
          {archivedCount != null && archivedCount > 0 ? (
            <span className="text-base-content/50"> ({archivedCount})</span>
          ) : (
            <span className="text-base-content/50"> {"(none)"}</span>
          )}
        </span>

        <input
          onChange={() => setShowArchived(!showArchived)}
          type="checkbox"
          className="toggle toggle-xs"
          checked={showArchived}
          disabled={archivedCount === 0}
        />
      </label>
    </div>
  );
}
