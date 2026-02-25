import { Modal } from "@/components/forms/Modal";
import { NewCollectionInner } from "@/features/admin/sections/collections/NewCollectionForm";
import { useCreateProjectCollection } from "../../contexts/CreateProjectCollectionContext";

const CreateProjectCollectionInner = () => {
  const { project, clear } = useCreateProjectCollection();
  return <NewCollectionInner projectId={project?.id} onClose={clear} />;
};

export const CreateProjectCollectionForm = () => (
  <Modal id="create_project_collection" title="Create project collection">
    <CreateProjectCollectionInner />
  </Modal>
);
