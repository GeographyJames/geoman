import { Select } from "@/components/forms/components/Select";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useCollections } from "@/hooks/api/useCollections";

const AddSiteFeatureInner = () => {
  const { data: collections } = useCollections();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formData = new FormData(e.currentTarget);
    const collection = formData.get("collection");
    const files = formData.getAll("files");
    // TODO: call post mutation with collection + files
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <Select name="collection" label="Select collection" required={true}>
        {collections?.map((c) => (
          <option key={c.id} value={c.id}>
            {`${c.title} (${c.geometry_type})`}
          </option>
        ))}
      </Select>
      <label className="form-control w-full pb-3">
        <span className="font-bold label">Select shapefiles</span>
        <input
          name="files"
          multiple={true}
          className="file-input file-input-bordered w-full"
          type="file"
          required
        />
      </label>
      <div className="modal-action">
        <CancelButton onClick={closeDialog} />
        <SubmitButton text="Add feature" />
      </div>
    </form>
  );
};

export const AddSiteFeatureForm = () => {
  return (
    <Modal id="add_site_feature" title="Add site feature">
      <AddSiteFeatureInner />
    </Modal>
  );
};
