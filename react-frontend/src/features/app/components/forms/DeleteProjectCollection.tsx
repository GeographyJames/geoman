import { Modal } from "@/components/forms/Modal";
import { DeleteCollectionInner } from "@/features/admin/sections/collections/DeleteCollectionForm";
import { useDeleteProjectCollection } from "../../contexts/DeleteProjectCollectionContext";

const MODAL_ID = "delete_project_collection";

const DeleteProjectCollectionInner = () => {
  const { collection, clear } = useDeleteProjectCollection();
  return <DeleteCollectionInner collection={collection} onClose={clear} />;
};

export const DeleteProjectCollectionForm = () => (
  <Modal id={MODAL_ID} title="Delete collection">
    <DeleteProjectCollectionInner />
  </Modal>
);
