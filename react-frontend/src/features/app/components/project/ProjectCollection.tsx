import { useProjectCollectionItems } from "@/hooks/api/useProjectCollectionItems";
import type { Collection } from "@/hooks/api/useProjectCollections";

export const ProjectCollection = ({
  collectionId,
  projectId,
}: {
  collectionId: number;
  projectId: number;
}) => {
  const { data } = useProjectCollectionItems({ projectId, collectionId });
  if (data) return <>{data.features.map((f) => f.id).join(",")}</>;
};
