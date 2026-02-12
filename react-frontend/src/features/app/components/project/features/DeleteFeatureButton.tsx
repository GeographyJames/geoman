import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";
import { useDeleteFeature } from "@/features/app/contexts/DeleteFeatureContext";

export const DeleteFeatureButton = ({
  feature,
}: {
  feature: ProjectCollectionItem;
}) => {
  const { requestDelete } = useDeleteFeature();
  return <button onClick={() => requestDelete(feature)}>delete</button>;
};
