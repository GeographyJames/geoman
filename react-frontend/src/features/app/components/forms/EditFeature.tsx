import { useEffect, useState } from "react";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { TextInput } from "@/components/forms/components/TextInput";
import { useEditFeature } from "@/features/app/contexts/EditFeatureContext";
import { usePatchProjectFeature } from "@/hooks/api/projectFeature.ts/usePatchProjectFeature";
import { ApiError } from "@/lib/api";

const EditFeatureInner = () => {
  const { feature, clear } = useEditFeature();
  const { mutate: patchFeature, isPending } = usePatchProjectFeature();
  const { addError, closeDialog } = useModal();
  const [name, setName] = useState("");
  const [primary, setPrimary] = useState(false);

  useEffect(() => {
    if (feature) {
      setName(feature.properties.name);
      setPrimary(feature.properties.is_primary);
    }
  }, [feature]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!feature) return;
    patchFeature(
      {
        projectId: feature.properties.project_id,
        collectionId: feature.properties.collection_id.toString(),
        id: feature.id,
        dto: { name, primary },
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
          addError(`Unable to update feature: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    setName(feature?.properties.name ?? "");
    setPrimary(feature?.properties.is_primary ?? false);
    closeDialog();
    clear();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {feature && (
        <>
          <TextInput
            name="name"
            label="Name"
            required
            value={name}
            onChange={setName}
          />
          <label className="label cursor-pointer justify-start gap-2">
            <input
              type="checkbox"
              className="checkbox"
              checked={primary}
              onChange={(e) => setPrimary(e.target.checked)}
            />
            <span className="label-text text-base-content">Set as primary</span>
          </label>
        </>
      )}
      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
        <SubmitButton
          disabled={
            name === feature?.properties.name &&
            primary === feature?.properties.is_primary
          }
          text="Save changes"
          loadingText="Saving..."
          loading={isPending}
        />
      </div>
    </form>
  );
};

export const EditFeatureForm = () => {
  return (
    <Modal
      id="edit_feature"
      title="Edit feature"
      onClose={useEditFeature().clear}
    >
      <EditFeatureInner />
    </Modal>
  );
};
