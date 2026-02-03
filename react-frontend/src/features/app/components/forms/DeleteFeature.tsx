import { ModalForm } from "@/components/forms/ModalForm";
import { useDeleteFeature } from "@/features/app/contexts/DeleteFeatureContext";
import { usePatchProjectFeature } from "@/hooks/api/projectFeature.ts/usePatchProjectFeature";
import { useFlash } from "../../contexts/FlashMessageContext";

export const DeleteFeatureForm = () => {
  const { feature, clear } = useDeleteFeature();
  const { mutate: patchFeature } = usePatchProjectFeature();
  const { addFlash } = useFlash();
  const handleSubmit = () => {
    if (feature) {
      patchFeature(
        {
          projectId: feature.properties.project_id,
          collectionId: feature.properties.collection_id.toString(),
          id: feature.id,
          dto: { status: "DELETED" },
        },
        {
          onError: (error) => {
            addFlash(`Unable to delete feature: ${error.message}`, "error");
          },
        },
      );
    }
  };

  return (
    <ModalForm
      id="delete_feature"
      title="Delete"
      onSubmit={handleSubmit}
      onReset={handleSubmit}
      onClose={clear}
      submitButtonText="Delete"
      submitButtonColour="btn-error"
    >
      {feature && (
        <p>
          Are you sure you want to permanently delete{" "}
          <span className="font-bold">{feature.properties.name}</span> from the{" "}
          {feature.properties.collection_title} collection?
        </p>
      )}
      <p>
        <span className="font-bold">This action cannot be undone.</span>
      </p>
    </ModalForm>
  );
};
