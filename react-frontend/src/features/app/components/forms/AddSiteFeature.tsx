import { Select } from "@/components/forms/components/Select";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useCollections } from "@/hooks/api/useCollections";
import { useForm } from "react-hook-form";
import { useEffect } from "react";
import { Shapefile } from "@/lib/shapefile";
import { usePostProjectFeature } from "@/hooks/api/usePostProjectFeature";
import { useAddFeature } from "../../contexts/AddFeatureContext";

const AddSiteFeatureInner = () => {
  const { project, clear } = useAddFeature();
  const { data: collections } = useCollections();
  const { addError, clearErrors, closeDialog } = useModal();
  const { mutate: postFeature, isPending } = usePostProjectFeature();
  const { register, watch, setValue, reset } = useForm();
  const files = watch("files") as FileList;

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!project) return;
    const formData = new FormData(e.currentTarget);
    const collectionId = Number(formData.get("collection"));
    const name = formData.get("name") as string;
    const shapefile = Shapefile.fromFieldValues({ files });
    if (typeof shapefile === "string") {
      addError(shapefile);
      return;
    }
    postFeature(
      { projectId: project.id, collectionId, name, shapefile },
      {
        onSuccess: () => {
          reset();
          closeDialog();
          clear();
        },
        onError: (error) => {
          addError(`Unable to add feature: ${error.message}`);
        },
      },
    );
  };

  useEffect(() => {
    if (files instanceof FileList && files.length > 0) {
      clearErrors();
      const result = Shapefile.fromFilesList(files);
      if (typeof result === "string") {
        addError(result);
        setValue("name", null);
        return;
      }
      setValue("name", result.filename);
    }
  }, [files]);

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <Select name="collection" label="Select collection" required={true}>
        {collections?.map((c) => (
          <option key={c.id} value={c.id}>
            {`${c.title} (${c.geometry_type})`}
          </option>
        ))}
      </Select>
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Shapefiles</legend>
        <input
          {...register("files")}
          multiple={true}
          className="file-input file-input-bordered w-full"
          type="file"
          required
        />
      </fieldset>
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Name</legend>
        <div className="flex items-center gap-2">
          <input
            className="input input-bordered w-full"
            {...register("name")}
            required
            type="text"
          />
        </div>
      </fieldset>
      <div className="modal-action">
        <CancelButton onClick={() => { reset(); closeDialog(); clear(); }} disabled={isPending} />
        <SubmitButton text="Add feature" loadingText="Adding..." loading={isPending} />
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
