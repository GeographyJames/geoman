import { usePatchCollection } from "@/hooks/api/usePatchCollection";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { ApiError } from "@/lib/api";

export interface CollectionForDelete {
  id: number;
  title: string;
}

export const DeleteCollectionInner = ({
  collection,
  onClose,
}: {
  collection: CollectionForDelete | null;
  onClose: () => void;
}) => {
  const { mutate: patchCollection, isPending } = usePatchCollection();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!collection) return;
    patchCollection(
      {
        id: collection.id,
        patch: { status: "DELETED" },
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
          addError(`Unable to delete collection: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    closeDialog();
    onClose();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {collection && (
        <p>
          Are you sure you want to permanently delete{" "}
          <span className="font-bold">{collection.title}</span>?
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

export const DeleteCollectionForm = ({
  collection,
  onClose,
}: {
  collection: CollectionForDelete | null;
  onClose: () => void;
}) => {
  return (
    <Modal id="delete_collection" title="Delete collection" onClose={onClose}>
      <DeleteCollectionInner collection={collection} onClose={onClose} />
    </Modal>
  );
};
