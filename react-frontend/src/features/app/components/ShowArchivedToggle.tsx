export default function ShowArchivedToggle({
  showArchived,
  setShowArchived,
}: {
  showArchived: boolean;
  setShowArchived: (val: boolean) => void;
}) {
  return (
    <div className="flex content-center">
      <label className="label cursor-pointer p-0">
        <input
          onChange={() => setShowArchived(!showArchived)}
          type="checkbox"
          className="toggle toggle-xs"
          checked={showArchived}
        />
        <span className="label-text text-xs">show archived</span>
      </label>
    </div>
  );
}
