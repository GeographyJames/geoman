import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";

import { useFlash } from "../../contexts/FlashMessageContext";
import { usePatchProjectFeature } from "@/hooks/api/projectFeature.ts/usePatchProjectFeature";

function SetPrimaryRadio({ item }: { item: ProjectCollectionItem }) {
  const { mutate: patchProject } = usePatchProjectFeature();
  const { addFlash } = useFlash();
  const handleClick = () => {
    {
      patchProject(
        {
          projectId: item.properties.project_id,
          collectionId: item.properties.collection_id.toString(),
          id: item.id,
          dto: { primary: true },
        },
        {
          onError: (error) => {
            addFlash(
              `Unable to set feature to primary: ${error.message}`,
              "error",
            );
          },
        },
      );
    }
  };
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
