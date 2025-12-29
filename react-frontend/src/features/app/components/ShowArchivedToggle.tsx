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
        <span className="label-text text-xs">show archived</span>
        <input
          onChange={() => setShowArchived(!showArchived)}
          type="checkbox"
          className="toggle toggle-xs"
          checked={showArchived}
        />
      </label>
    </div>
  );
}
