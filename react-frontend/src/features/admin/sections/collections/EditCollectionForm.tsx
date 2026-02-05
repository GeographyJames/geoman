import { useForm } from "react-hook-form";
import type { Collection } from "@/hooks/api/useCollections";
import { usePatchCollection } from "@/hooks/api/usePatchCollection";

interface EditFormData {
  title: string;
  description: string;
}

export const EditCollectionForm = ({
  editingCollection,
  setEditingCollection,
}: {
  editingCollection: Collection;
  setEditingCollection: React.Dispatch<React.SetStateAction<Collection | null>>;
}) => {
  const patchCollectionMutation = usePatchCollection();

  const { register, handleSubmit, formState: { isDirty, dirtyFields } } = useForm<EditFormData>({
    defaultValues: {
      title: editingCollection.title,
      description: editingCollection.description ?? "",
    },
  });

  const close = () => setEditingCollection(null);

  const onSubmit = async (data: EditFormData) => {
    try {
      await patchCollectionMutation.mutateAsync({
        id: editingCollection.id,
        patch: {
          title: dirtyFields.title ? data.title : undefined,
          description: dirtyFields.description
            ? data.description || null
            : undefined,
        },
      });
      close();
    } catch (error) {
      console.error("Failed to update collection:", error);
      alert("Failed to update collection. Please try again.");
    }
  };

  return (
    <dialog className="modal modal-open">
      <div className="modal-box">
        <h3 className="font-bold text-lg mb-4">Edit collection</h3>
        <form onSubmit={handleSubmit(onSubmit)}>
          <div className="form-control mb-4">
            <label className="label">
              <span className="label-text font-medium">Title</span>
            </label>
            <input
              type="text"
              {...register("title", { required: true })}
              className="input input-bordered"
              autoFocus
            />
          </div>
          <div className="form-control mb-6">
            <label className="label">
              <span className="label-text font-medium">
                Description{" "}
                <span className="text-base-content/50 font-normal">
                  (optional)
                </span>
              </span>
            </label>
            <textarea
              {...register("description")}
              placeholder="Describe this collection..."
              className="textarea textarea-bordered"
              rows={3}
            />
          </div>
          <div className="modal-action">
            <button type="button" onClick={close} className="btn">
              Cancel
            </button>
            <button
              type="submit"
              disabled={!isDirty || patchCollectionMutation.isPending}
              className="btn btn-primary"
            >
              {patchCollectionMutation.isPending ? (
                <>
                  <span className="loading loading-spinner loading-sm"></span>
                  Saving...
                </>
              ) : (
                "Save changes"
              )}
            </button>
          </div>
        </form>
      </div>
      <form method="dialog" className="modal-backdrop">
        <button onClick={close}>close</button>
      </form>
    </dialog>
  );
};
