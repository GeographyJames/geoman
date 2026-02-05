import { useCreateApiKey } from "@/hooks/api/useCreateApiKey";
import { useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { AlertCircle, Copy } from "lucide-react";

export const CreateKeyForm = ({
  setShowCreateModal,
}: {
  setShowCreateModal: React.Dispatch<React.SetStateAction<boolean>>;
}) => {
  const [newKeyName, setNewKeyName] = useState("");
  const [generatedKey, setGeneratedKey] = useState<string | null>(null);
  const createApiKeyMutation = useCreateApiKey();
  const queryClient = useQueryClient();
  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };
  const handleCreateKey = async () => {
    try {
      const result = await createApiKeyMutation.mutateAsync({
        key_name: newKeyName,
      });
      setGeneratedKey(result.api_key);
      setNewKeyName("");
      queryClient.invalidateQueries({ queryKey: ["apiKeys"] });
    } catch (error) {
      console.error("Failed to create API key:", error);
      alert("Failed to create API key. Please try again.");
    }
  };

  return (
    <dialog className="modal modal-open">
      <div className="modal-box">
        {!generatedKey ? (
          <>
            <h3 className="font-bold text-lg mb-4">Create new API key</h3>
            <p className="text-sm text-base-content/70 mb-4">
              Give your API key a descriptive name to help you identify it
              later.
            </p>
            <div className="form-control mb-6">
              <label className="label">
                <span className="label-text font-medium">Key name</span>
              </label>
              <input
                type="text"
                value={newKeyName}
                onChange={(e) => setNewKeyName(e.target.value)}
                placeholder="e.g., My QGIS Desktop"
                className="input input-bordered"
                autoFocus
              />
            </div>
            <div className="modal-action">
              <button
                onClick={() => {
                  setShowCreateModal(false);
                  setNewKeyName("");
                }}
                className="btn"
              >
                Cancel
              </button>
              <button
                onClick={handleCreateKey}
                disabled={!newKeyName.trim() || createApiKeyMutation.isPending}
                className="btn btn-primary"
              >
                {createApiKeyMutation.isPending ? (
                  <>
                    <span className="loading loading-spinner loading-sm"></span>
                    Creating...
                  </>
                ) : (
                  "Create key"
                )}
              </button>
            </div>
          </>
        ) : (
          <>
            <h3 className="font-bold text-lg mb-4">API key created</h3>
            <div className="alert alert-warning mb-4">
              <AlertCircle size={20} />
              <div className="text-sm">
                <div className="font-semibold">Save this key now!</div>
                <div>
                  This is the only time you'll see this key. Make sure to copy
                  it somewhere safe.
                </div>
              </div>
            </div>
            <div className="form-control mb-6">
              <label className="label">
                <span className="label-text font-medium">Your API Key</span>
              </label>
              <div className="join w-full">
                <input
                  type="text"
                  value={generatedKey}
                  readOnly
                  className="input input-bordered join-item flex-1 font-mono text-sm"
                />
                <button
                  onClick={() => copyToClipboard(generatedKey)}
                  className="btn join-item"
                  title="Copy to clipboard"
                >
                  <Copy size={18} />
                </button>
              </div>
            </div>
            <div className="modal-action">
              <button
                onClick={() => {
                  setShowCreateModal(false);
                  setGeneratedKey(null);
                }}
                className="btn btn-primary w-full"
              >
                Done
              </button>
            </div>
          </>
        )}
      </div>
      <form method="dialog" className="modal-backdrop">
        <button
          onClick={() => {
            setShowCreateModal(false);
            setGeneratedKey(null);
            setNewKeyName("");
          }}
        >
          close
        </button>
      </form>
    </dialog>
  );
};
//
