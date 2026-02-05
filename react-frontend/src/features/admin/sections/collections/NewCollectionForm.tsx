import { useState } from "react";
import { useCreateCollection } from "@/hooks/api/useCreateCollection";
const GEOMETRY_TYPES = [
  "Point",
  "LineString",
  "Polygon",
  "MultiPoint",
  "MultiLineString",
  "MultiPolygon",
  "GeometryCollection",
] as const;

export const NewCollectionForm = ({
  setShowCreateModal,
}: {
  setShowCreateModal: React.Dispatch<React.SetStateAction<boolean>>;
}) => {
  const [newTitle, setNewTitle] = useState("");
  const [newGeometryType, setNewGeometryType] = useState<string>("Point");
  const [newDescription, setNewDescription] = useState("");
  const createCollectionMutation = useCreateCollection();

  const handleCreate = async () => {
    try {
      await createCollectionMutation.mutateAsync({
        title: newTitle,
        geometry_type: newGeometryType,
        description: newDescription || undefined,
      });
      setShowCreateModal(false);
      setNewTitle("");
      setNewGeometryType("Point");
      setNewDescription("");
    } catch (error) {
      console.error("Failed to create collection:", error);
      alert("Failed to create collection. Please try again.");
    }
  };
  return (
    <dialog className="modal modal-open">
      <div className="modal-box">
        <h3 className="font-bold text-lg mb-4">Create new collection</h3>
        <p className="text-sm text-base-content/70 mb-4">
          Define a new geospatial data collection.
        </p>
        <div className="form-control mb-4">
          <label className="label">
            <span className="label-text font-medium">Title</span>
          </label>
          <input
            type="text"
            value={newTitle}
            onChange={(e) => setNewTitle(e.target.value)}
            placeholder="e.g., Survey Sites"
            className="input input-bordered"
            autoFocus
          />
        </div>
        <div className="form-control mb-4">
          <label className="label">
            <span className="label-text font-medium">Geometry Type</span>
          </label>
          <select
            value={newGeometryType}
            onChange={(e) => setNewGeometryType(e.target.value)}
            className="select select-bordered"
          >
            {GEOMETRY_TYPES.map((type_) => (
              <option key={type_} value={type_}>
                {type_}
              </option>
            ))}
          </select>
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
            value={newDescription}
            onChange={(e) => setNewDescription(e.target.value)}
            placeholder="Describe this collection..."
            className="textarea textarea-bordered"
            rows={3}
          />
        </div>
        <div className="modal-action">
          <button
            onClick={() => {
              setShowCreateModal(false);
              setNewTitle("");
              setNewGeometryType("Point");
              setNewDescription("");
            }}
            className="btn"
          >
            Cancel
          </button>
          <button
            onClick={handleCreate}
            disabled={!newTitle.trim() || createCollectionMutation.isPending}
            className="btn btn-primary"
          >
            {createCollectionMutation.isPending ? (
              <>
                <span className="loading loading-spinner loading-sm"></span>
                Creating...
              </>
            ) : (
              "Create collection"
            )}
          </button>
        </div>
      </div>
      <form method="dialog" className="modal-backdrop">
        <button
          onClick={() => {
            setShowCreateModal(false);
            setNewTitle("");
            setNewGeometryType("Point");
            setNewDescription("");
          }}
        >
          close
        </button>
      </form>
    </dialog>
  );
};
