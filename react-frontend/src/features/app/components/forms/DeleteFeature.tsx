import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useDeleteFeature } from "@/features/app/contexts/DeleteFeatureContext";
import { usePatchProjectFeature } from "@/hooks/api/projectFeature.ts/usePatchProjectFeature";
import { ApiError } from "@/lib/api";

const DeleteFeatureInner = () => {
  const { feature, clear } = useDeleteFeature();
  const { mutate: patchFeature, isPending } = usePatchProjectFeature();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!feature) return;
    patchFeature(
      {
        projectId: feature.properties.project_id,
        collectionId: feature.properties.collection_id.toString(),
        id: feature.id,
        dto: { status: "DELETED" },
      },
      {
        onSuccess: () => {
          closeDialog();
          clear();
        },
        onError: (error) => {
          const message =
            error instanceof ApiError && error.status === 500
              ? "internal server error"
              : error.message;
          addError(`Unable to delete feature: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    closeDialog();
    clear();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
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
      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
        <SubmitButton
          text="Delete"
          colour="btn-error"
          loadingText="Deleting..."
          loading={isPending}
        />
      </div>
    </form>
  );
};

export const DeleteFeatureForm = () => {
  return (
    <Modal
      id="delete_feature"
      title="Delete project feature"
      onClose={useDeleteFeature().clear}
    >
      <DeleteFeatureInner />
    </Modal>
  );
};
