import { Plus, AlertCircle, Layers, Pencil, Trash2 } from "lucide-react";
import { useState } from "react";
import { useCollections, type Collection } from "@/hooks/api/useCollections";
import { useCurrentUser } from "@/hooks/api/useCurrentUser";
import UserInitials from "@/components/UserInitials";
import { dateFormat } from "@/constants";

import { NewCollectionForm } from "./NewCollectionForm";
import { EditCollectionForm } from "./EditCollectionForm";
import { DeleteCollectionForm } from "./DeleteCollectionForm";

export default function CollectionsSection() {
  const openCreateModal = () => {
    const el = document.getElementById("new_collection");
    if (el instanceof HTMLDialogElement) el.showModal();
  };

  const [editingCollection, setEditingCollection] = useState<Collection | null>(
    null,
  );
  const [deletingCollection, setDeletingCollection] =
    useState<Collection | null>(null);

  const { data: collections = [], isLoading, error } = useCollections();
  const { data: currentUser } = useCurrentUser();
  return (
    <>
      {/* Page Header */}
      <div className="mb-6">
        <div className="flex justify-between gap-2">
          <div>
            <h1 className="text-2xl font-semibold mb-1">Global Collections</h1>
            <p className="text-base-content/70">
              Collections available across all projects
            </p>
          </div>
          {currentUser?.isAdmin && (
            <button
              onClick={() => openCreateModal()}
              className="btn btn-primary gap-2"
            >
              <Plus size={20} />
              New Collection
            </button>
          )}
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
            <p className="text-base-content/70">
              No collections have been created yet.
            </p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="table">
              <thead>
                <tr>
                  <th>Title</th>
                  <th>Geometry Type</th>
                  <th>Active</th>
                  <th>Archived</th>
                  <th>Added By</th>
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
                    <td className="text-sm">
                      {collection.active_feature_count}
                    </td>
                    <td className="text-sm">
                      {collection.archived_feature_count}
                    </td>
                    <td>
                      <UserInitials
                        firstName={collection.added_by_first_name}
                        lastName={collection.added_by_last_name}
                        message={`${collection.added_by_first_name} ${collection.added_by_last_name} · ${dateFormat.format(new Date(collection.added))}`}
                      />
                    </td>
                    <td className="text-sm text-base-content/70">
                      {collection.description || (
                        <span className="opacity-50">-</span>
                      )}
                    </td>
                    <td>
                      {currentUser?.isAdmin && (
                        <div className="flex gap-1">
                          <button
                            onClick={() => {
                              setEditingCollection(collection);
                              const el =
                                document.getElementById("edit_collection");
                              if (el instanceof HTMLDialogElement)
                                el.showModal();
                            }}
                            className="btn btn-ghost btn-sm gap-1"
                          >
                            <Pencil size={14} />
                            Edit
                          </button>
                          <button
                            onClick={() => {
                              setDeletingCollection(collection);
                              const el =
                                document.getElementById("delete_collection");
                              if (el instanceof HTMLDialogElement)
                                el.showModal();
                            }}
                            className={`btn btn-ghost btn-sm gap-1 ${
                              collection.active_feature_count > 0 ||
                              collection.archived_feature_count > 0
                                ? "text-base-content/30"
                                : "text-error"
                            }`}
                            disabled={
                              collection.active_feature_count > 0 ||
                              collection.archived_feature_count > 0
                            }
                          >
                            <Trash2 size={14} />
                            Delete
                          </button>
                        </div>
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
      <NewCollectionForm />

      {/* Edit Collection Modal */}
      <EditCollectionForm
        collection={editingCollection}
        onClose={() => setEditingCollection(null)}
      />

      {/* Delete Collection Modal */}
      <DeleteCollectionForm
        collection={deletingCollection}
        onClose={() => setDeletingCollection(null)}
      />
    </>
  );
}
