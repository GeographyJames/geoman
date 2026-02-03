import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";
import { useDeleteFeature } from "@/features/app/contexts/DeleteFeatureContext";

export const DeleteFeatureButton = ({
  feature,
}: {
  feature: ProjectCollectionItem;
}) => {
  const { requestDelete } = useDeleteFeature();
  return (
    <button
      onClick={() => requestDelete(feature)}
      disabled={feature.properties.is_primary}
      className={feature.properties.is_primary ? "text-base-content/50" : ""}
    >
      delete
    </button>
  );
};
