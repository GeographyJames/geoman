import { useForm } from "react-hook-form";
import { useCreateCollection } from "@/hooks/api/useCreateCollection";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { ApiError } from "@/lib/api";

const GEOMETRY_TYPES = [
  "Point",
  "LineString",
  "Polygon",
  "MultiPoint",
  "MultiLineString",
  "MultiPolygon",
] as const;

interface NewCollectionFormData {
  title: string;
  geometry_type: string;
  description: string;
}

export const NewCollectionInner = ({
  projectId,
  onClose,
}: {
  projectId?: number;
  onClose?: () => void;
} = {}) => {
  const { mutate: createCollection, isPending } = useCreateCollection();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    reset,
    formState: { isValid },
  } = useForm<NewCollectionFormData>({
    defaultValues: {
      title: "",
      geometry_type: "Point",
      description: "",
    },
  });

  const onSubmit = (data: NewCollectionFormData) => {
    createCollection(
      {
        title: data.title,
        geometry_type: data.geometry_type,
        description: data.description || undefined,
        project_id: projectId,
      },
      {
        onSuccess: () => {
          reset();
          closeDialog();
          onClose?.();
        },
        onError: (error) => {
          const message =
            error instanceof ApiError && error.status === 500
              ? "internal server error"
              : error.message;
          addError(`Unable to create collection: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    reset();
    closeDialog();
    onClose?.();
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Title</legend>
        <input
          type="text"
          {...register("title", { required: true })}
          placeholder="e.g., Survey Sites"
          className="input input-bordered w-full"
          autoFocus
        />
      </fieldset>
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Geometry Type</legend>
        <select
          {...register("geometry_type")}
          className="select select-bordered w-full"
        >
          {GEOMETRY_TYPES.map((type_) => (
            <option key={type_} value={type_}>
              {type_}
            </option>
          ))}
        </select>
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
          text="Create collection"
          loadingText="Creating..."
          loading={isPending}
          disabled={!isValid}
        />
      </div>
    </form>
  );
};

export const NewCollectionForm = () => {
  return (
    <Modal id="new_collection" title="Create new collection">
      <NewCollectionInner />
    </Modal>
  );
};
