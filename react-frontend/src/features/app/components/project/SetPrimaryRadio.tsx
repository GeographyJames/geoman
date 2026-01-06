import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";

function SetPrimaryRadio({ item }: { item: ProjectCollectionItem }) {
  const handleClick = () => {};
  return (
    <div
      className={`${
        item.properties.status === "ARCHIVED" ? "tooltip tooltip-left flex" : ""
      } flex`}
      data-tip={`Unable to set an archived ${item.properties.collection_id} as primary`}
    >
      <input
        onChange={handleClick}
        className="radio-sm radio bg-base-100"
        type="radio"
        checked={item.properties.is_primary}
        disabled={item.properties.status === "ARCHIVED"}
        id={`${item.id}`}
      />
    </div>
  );
}

export default SetPrimaryRadio;
