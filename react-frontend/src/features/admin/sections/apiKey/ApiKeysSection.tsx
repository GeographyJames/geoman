import { Key, Plus, AlertCircle, Trash2 } from "lucide-react";
import { useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useApiKeys } from "@/hooks/api/useApiKeys";
import { useRevokeApiKey } from "@/hooks/api/useRevokeApiKey";
import { useRenewApiKey } from "@/hooks/api/useRenewApiKey";
import { CreateKeyForm } from "./CreateKeyForm";

export default function ApiKeysSection() {
  const [showCreateModal, setShowCreateModal] = useState(false);
  const queryClient = useQueryClient();
  const { data: apiKeys = [], isLoading, error } = useApiKeys();
  const revokeApiKeyMutation = useRevokeApiKey();
  const renewApiKeyMutation = useRenewApiKey();
  const handleRevokeKey = async (keyId: number) => {
    if (
      !confirm(
        "Are you sure you want to revoke this API key? This action cannot be undone.",
      )
    ) {
      return;
    }

    try {
      await revokeApiKeyMutation.mutateAsync(keyId);
      queryClient.invalidateQueries({ queryKey: ["apiKeys"] });
    } catch (error) {
      console.error("Failed to revoke API key:", error);
      alert("Failed to revoke API key. Please try again.");
    }
  };

  const handleRenewKey = async (keyId: number) => {
    try {
      await renewApiKeyMutation.mutateAsync(keyId);
      queryClient.invalidateQueries({ queryKey: ["apiKeys"] });
    } catch (error) {
      console.error("Failed to renew API key:", error);
      alert("Failed to renew API key. Please try again.");
    }
  };

  const isExpired = (expiryDate: string) => {
    return new Date(expiryDate) < new Date();
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  };

  return (
    <>
      {/* Page Header */}
      <div className="mb-6">
        <div className="flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-semibold mb-1">API Keys</h1>
            <p className="text-base-content/70">
              Manage your API keys for QGIS and other applications
            </p>
          </div>
          <button
            onClick={() => setShowCreateModal(true)}
            className="btn btn-primary gap-2"
          >
            <Plus size={20} />
            New API Key
          </button>
        </div>
      </div>

      {/* API Keys Table */}
      <div className="card bg-base-100 border border-base-300">
        {isLoading ? (
          <div className="card-body items-center text-center py-12">
            <span className="loading loading-spinner loading-lg"></span>
            <p className="mt-4 text-base-content/70">Loading API keys...</p>
          </div>
        ) : error ? (
          <div className="card-body items-center text-center py-12">
            <AlertCircle size={48} className="text-error mb-4" />
            <h3 className="text-lg font-semibold mb-2">
              Failed to load API keys
            </h3>
            <p className="text-base-content/70">
              {error instanceof Error ? error.message : "An error occurred"}
            </p>
          </div>
        ) : apiKeys.length === 0 ? (
          <div className="card-body items-center text-center py-12">
            <Key size={48} className="opacity-30 mb-4" />
            <h3 className="text-lg font-semibold mb-2">No API keys</h3>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Created</th>
                  <th>Last Used</th>
                  <th>Last IP</th>
                  <th>User Agent</th>
                  <th>Expires</th>
                  <th>Status</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                {apiKeys.map((key) => (
                  <tr key={key.id}>
                    <td>
                      <div className="flex items-center gap-2">
                        <Key size={16} className="opacity-50" />
                        <span className="font-medium">{key.name}</span>
                      </div>
                    </td>
                    <td className="text-sm">{formatDate(key.created)}</td>
                    <td className="text-sm">
                      {key.last_used ? formatDate(key.last_used) : "Never"}
                    </td>
                    <td className="text-sm">
                      {key.last_used_ip ? (
                        <span className="font-mono text-xs">
                          {key.last_used_ip}
                        </span>
                      ) : (
                        <span className="opacity-50">-</span>
                      )}
                    </td>
                    <td
                      className="text-sm max-w-xs truncate"
                      title={key.last_used_user_agent || undefined}
                    >
                      {key.last_used_user_agent ? (
                        <span className="text-xs">
                          {key.last_used_user_agent}
                        </span>
                      ) : (
                        <span className="opacity-50">-</span>
                      )}
                    </td>
                    <td className="text-sm">{formatDate(key.expiry)}</td>
                    <td>
                      {isExpired(key.expiry) ? (
                        <div className="badge badge-warning badge-sm">
                          Expired
                        </div>
                      ) : (
                        <div className="badge badge-success badge-sm">
                          Active
                        </div>
                      )}
                    </td>
                    <td>
                      <div className="flex gap-2">
                        <button
                          onClick={() => handleRenewKey(key.id)}
                          disabled={renewApiKeyMutation.isPending}
                          className="btn btn-ghost btn-sm text-primary gap-1"
                        >
                          {renewApiKeyMutation.isPending ? (
                            <span className="loading loading-spinner loading-xs"></span>
                          ) : (
                            <Plus size={14} />
                          )}
                          Renew
                        </button>
                        <button
                          onClick={() => handleRevokeKey(key.id)}
                          disabled={revokeApiKeyMutation.isPending}
                          className="btn btn-ghost btn-sm text-error gap-1"
                        >
                          {revokeApiKeyMutation.isPending ? (
                            <span className="loading loading-spinner loading-xs"></span>
                          ) : (
                            <Trash2 size={14} />
                          )}
                          Revoke
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Info Box */}
      <div className="alert mt-6">
        <AlertCircle size={20} />
        <div>
          <h3 className="font-semibold">About API Keys</h3>
          <div className="text-sm opacity-80">
            API keys allow applications like QGIS to access your geospatial
            data. Keys expire after 6 months and can be revoked at any time.
          </div>
        </div>
      </div>

      {/* Create Key Modal */}
      {showCreateModal && (
        <CreateKeyForm setShowCreateModal={setShowCreateModal} />
      )}
    </>
  );
}
