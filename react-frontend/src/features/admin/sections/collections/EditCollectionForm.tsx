import { useForm } from "react-hook-form";
import { usePatchCollection } from "@/hooks/api/usePatchCollection";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { ApiError } from "@/lib/api";

export interface CollectionForEdit {
  id: number;
  title: string;
  description?: string | null;
}

interface EditFormData {
  title: string;
  description: string;
}

export const EditCollectionInner = ({
  collection,
  onClose,
}: {
  collection: CollectionForEdit | null;
  onClose: () => void;
}) => {
  const { mutate: patchCollection, isPending } = usePatchCollection();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    formState: { isDirty, dirtyFields },
  } = useForm<EditFormData>({
    values: collection
      ? {
          title: collection.title,
          description: collection.description ?? "",
        }
      : undefined,
  });

  const onSubmit = (data: EditFormData) => {
    if (!collection) return;
    patchCollection(
      {
        id: collection.id,
        patch: {
          title: dirtyFields.title ? data.title : undefined,
          description: dirtyFields.description
            ? data.description || null
            : undefined,
        },
      },
      {
        onSuccess: () => {
          closeDialog();
          onClose();
        },
        onError: (error) => {
          const message =
            error instanceof ApiError && error.status === 500
              ? "internal server error"
              : error.message;
          addError(`Unable to update collection: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    closeDialog();
    onClose();
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Title</legend>
        <input
          type="text"
          {...register("title", { required: true })}
          className="input input-bordered w-full"
          autoFocus
        />
      </fieldset>
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">
          Description{" "}
          <span className="text-base-content/50 font-normal">(optional)</span>
        </legend>
        <textarea
          {...register("description")}
          placeholder="Describe this collection..."
          className="textarea textarea-bordered w-full"
          rows={3}
        />
      </fieldset>
      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
        <SubmitButton
          text="Save changes"
          loadingText="Saving..."
          loading={isPending}
          disabled={!isDirty}
        />
      </div>
    </form>
  );
};

export const EditCollectionForm = ({
  collection,
  onClose,
}: {
  collection: CollectionForEdit | null;
  onClose: () => void;
}) => {
  return (
    <Modal id="edit_collection" title="Edit collection" onClose={onClose}>
      <EditCollectionInner collection={collection} onClose={onClose} />
    </Modal>
  );
};
