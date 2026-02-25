import { Modal } from "@/components/forms/Modal";
import { EditCollectionInner } from "@/features/admin/sections/collections/EditCollectionForm";
import { useEditProjectCollection } from "../../contexts/EditProjectCollectionContext";

const MODAL_ID = "edit_project_collection";

const EditProjectCollectionInner = () => {
  const { collection, clear } = useEditProjectCollection();
  return <EditCollectionInner collection={collection} onClose={clear} />;
};

export const EditProjectCollectionForm = () => (
  <Modal id={MODAL_ID} title="Edit collection">
    <EditProjectCollectionInner />
  </Modal>
);
