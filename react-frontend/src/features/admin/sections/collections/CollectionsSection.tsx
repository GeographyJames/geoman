import { Plus, AlertCircle, Layers, Pencil } from "lucide-react";
import { useState } from "react";
import { useCollections, type Collection } from "@/hooks/api/useCollections";

import { NewCollectionForm } from "./NewCollectionForm";
import { EditCollectionForm } from "./EditCollectionForm";

export default function CollectionsSection() {
  const [showCreateModal, setShowCreateModal] = useState(false);

  const [editingCollection, setEditingCollection] = useState<Collection | null>(
    null,
  );

  const { data: collections = [], isLoading, error } = useCollections();

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
            <p className="mt-4 text-base-content/70">Loading collections...</p>
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
                  <th>Features</th>
                  <th>Description</th>
                  <th></th>
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
                    <td className="text-sm">{collection.feature_count}</td>
                    <td className="text-sm text-base-content/70">
                      {collection.description || (
                        <span className="opacity-50">-</span>
                      )}
                    </td>
                    <td>
                      <button
                        onClick={() => setEditingCollection(collection)}
                        className="btn btn-ghost btn-sm gap-1"
                      >
                        <Pencil size={14} />
                        Edit
                      </button>
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
        <NewCollectionForm setShowCreateModal={setShowCreateModal} />
      )}

      {/* Edit Collection Modal */}
      {editingCollection && (
        <EditCollectionForm
          editingCollection={editingCollection}
          setEditingCollection={setEditingCollection}
        />
      )}
    </>
  );
}
