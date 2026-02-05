import { Plus, AlertCircle, Layers } from "lucide-react";
import { useState } from "react";
import { useCollections } from "@/hooks/api/useCollections";
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

export default function CollectionsSection() {
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [newTitle, setNewTitle] = useState("");
  const [newGeometryType, setNewGeometryType] = useState<string>("Point");
  const [newDescription, setNewDescription] = useState("");

  const { data: collections = [], isLoading, error } = useCollections();
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
    <>
      {/* Page Header */}
      <div className="mb-6">
        <div className="flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-semibold mb-1">Collections</h1>
            <p className="text-base-content/70">
              Manage your geospatial data collections
            </p>
          </div>
          <button
            onClick={() => setShowCreateModal(true)}
            className="btn btn-primary gap-2"
          >
            <Plus size={20} />
            New Collection
          </button>
        </div>
      </div>

      {/* Collections Table */}
      <div className="card bg-base-100 border border-base-300">
        {isLoading ? (
          <div className="card-body items-center text-center py-12">
            <span className="loading loading-spinner loading-lg"></span>
            <p className="mt-4 text-base-content/70">
              Loading collections...
            </p>
          </div>
        ) : error ? (
          <div className="card-body items-center text-center py-12">
            <AlertCircle size={48} className="text-error mb-4" />
            <h3 className="text-lg font-semibold mb-2">
              Failed to load collections
            </h3>
            <p className="text-base-content/70">
              {error instanceof Error ? error.message : "An error occurred"}
            </p>
          </div>
        ) : collections.length === 0 ? (
          <div className="card-body items-center text-center py-12">
            <Layers size={48} className="opacity-30 mb-4" />
            <h3 className="text-lg font-semibold mb-2">No collections</h3>
            <p className="text-base-content/70 mb-4">
              Get started by creating your first collection
            </p>
            <button
              onClick={() => setShowCreateModal(true)}
              className="btn btn-sm btn-primary gap-2"
            >
              <Plus size={16} />
              Create your first collection
            </button>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="table">
              <thead>
                <tr>
                  <th>Title</th>
                  <th>Geometry Type</th>
                  <th>Description</th>
                </tr>
              </thead>
              <tbody>
                {collections.map((collection) => (
                  <tr key={collection.id}>
                    <td>
                      <div className="flex items-center gap-2">
                        <Layers size={16} className="opacity-50" />
                        <span className="font-medium">{collection.title}</span>
                      </div>
                    </td>
                    <td>
                      <div className="badge badge-outline badge-sm">
                        {collection.geometry_type}
                      </div>
                    </td>
                    <td className="text-sm text-base-content/70">
                      {collection.description || (
                        <span className="opacity-50">-</span>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Create Collection Modal */}
      {showCreateModal && (
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
                disabled={
                  !newTitle.trim() || createCollectionMutation.isPending
                }
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
      )}
    </>
  );
}
