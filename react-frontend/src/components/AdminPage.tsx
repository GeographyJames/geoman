import { SignedIn, UserButton, useUser } from "@clerk/clerk-react";
import { Map, Key, Plus, Trash2, Copy, AlertCircle } from "lucide-react";
import { useState } from "react";
import { useCreateApiKey } from "@/hooks/api/useCreateApiKey";

interface ApiKey {
  id: number;
  name: string;
  created: string;
  last_used: string | null;
  expiry: string;
  revoked: boolean;
}

export default function AdminPage() {
  const { user } = useUser();
  const [apiKeys, setApiKeys] = useState<ApiKey[]>([]);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [newKeyName, setNewKeyName] = useState("");
  const [generatedKey, setGeneratedKey] = useState<string | null>(null);
  const [copiedKeyId, setCopiedKeyId] = useState<number | null>(null);

  const createApiKeyMutation = useCreateApiKey();

  const handleCreateKey = async () => {
    try {
      const result = await createApiKeyMutation.mutateAsync({
        key_name: newKeyName,
      });
      setGeneratedKey(result.api_key);
      setNewKeyName("");
    } catch (error) {
      console.error("Failed to create API key:", error);
      alert("Failed to create API key. Please try again.");
    }
  };

  const handleRevokeKey = async (keyId: number) => {
    // TODO: Connect to backend API
    console.log("Revoking key:", keyId);
    setApiKeys(apiKeys.map(key =>
      key.id === keyId ? { ...key, revoked: true } : key
    ));
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    // Show copied feedback
    setTimeout(() => setCopiedKeyId(null), 2000);
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-600 to-indigo-700">
      <header className="p-6 border-b border-white/10">
        <div className="max-w-7xl mx-auto flex justify-between items-center">
          <div className="flex items-center gap-2 text-white">
            <Map size={32} />
            <h1 className="text-2xl font-bold">GeoMan Admin</h1>
          </div>
          <SignedIn>
            <UserButton />
          </SignedIn>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-6 py-8">
        <div className="bg-white rounded-xl shadow-2xl p-8">
          <div className="flex justify-between items-center mb-6">
            <div>
              <h2 className="text-3xl font-bold text-gray-800">API Keys</h2>
              <p className="text-gray-600 mt-1">
                Manage your API keys for QGIS and other applications
              </p>
            </div>
            <button
              onClick={() => setShowCreateModal(true)}
              className="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-purple-500 to-indigo-600 text-white rounded-lg hover:from-purple-600 hover:to-indigo-700 transition font-semibold"
            >
              <Plus size={20} />
              Create New Key
            </button>
          </div>

          {/* API Keys List */}
          <div className="space-y-4">
            {apiKeys.length === 0 ? (
              <div className="text-center py-12 text-gray-500">
                <Key size={48} className="mx-auto mb-4 opacity-50" />
                <p className="text-lg">No API keys yet</p>
                <p className="text-sm">Create your first API key to get started</p>
              </div>
            ) : (
              apiKeys.map((key) => (
                <div
                  key={key.id}
                  className={`border rounded-lg p-4 ${
                    key.revoked
                      ? "bg-gray-50 border-gray-300"
                      : "bg-white border-gray-200"
                  }`}
                >
                  <div className="flex justify-between items-start">
                    <div className="flex-1">
                      <div className="flex items-center gap-2">
                        <h3 className="text-lg font-semibold text-gray-800">
                          {key.name}
                        </h3>
                        {key.revoked && (
                          <span className="px-2 py-1 bg-red-100 text-red-700 text-xs font-semibold rounded">
                            Revoked
                          </span>
                        )}
                      </div>
                      <div className="mt-2 space-y-1 text-sm text-gray-600">
                        <p>Created: {formatDate(key.created)}</p>
                        <p>
                          Last used:{" "}
                          {key.last_used
                            ? formatDate(key.last_used)
                            : "Never"}
                        </p>
                        <p>Expires: {formatDate(key.expiry)}</p>
                      </div>
                    </div>
                    {!key.revoked && (
                      <button
                        onClick={() => handleRevokeKey(key.id)}
                        className="flex items-center gap-2 px-3 py-2 text-red-600 hover:bg-red-50 rounded-lg transition"
                      >
                        <Trash2 size={16} />
                        Revoke
                      </button>
                    )}
                  </div>
                </div>
              ))
            )}
          </div>
        </div>

        {/* Quick Links */}
        <div className="mt-8 bg-white rounded-xl shadow-2xl p-8">
          <h3 className="text-2xl font-bold mb-6 text-gray-800">Quick Links</h3>
          <div className="grid md:grid-cols-2 gap-4">
            <a
              href="/"
              className="block p-4 bg-gradient-to-r from-purple-500 to-indigo-600 text-white rounded-lg hover:from-purple-600 hover:to-indigo-700 transition"
            >
              <h4 className="font-bold mb-1">Home</h4>
              <p className="text-sm text-purple-100">Return to landing page</p>
            </a>
            <a
              href="/ogcapi"
              className="block p-4 bg-gradient-to-r from-indigo-500 to-purple-600 text-white rounded-lg hover:from-indigo-600 hover:to-purple-700 transition"
            >
              <h4 className="font-bold mb-1">OGC API Features</h4>
              <p className="text-sm text-purple-100">Access the API</p>
            </a>
            <a
              href="/docs"
              className="block p-4 bg-gradient-to-r from-purple-500 to-pink-600 text-white rounded-lg hover:from-purple-600 hover:to-pink-700 transition"
            >
              <h4 className="font-bold mb-1">API Documentation</h4>
              <p className="text-sm text-purple-100">Interactive API reference</p>
            </a>
            <a
              href="/book"
              className="block p-4 bg-gradient-to-r from-pink-500 to-purple-600 text-white rounded-lg hover:from-pink-600 hover:to-purple-700 transition"
            >
              <h4 className="font-bold mb-1">Documentation</h4>
              <p className="text-sm text-purple-100">User guide and tutorials</p>
            </a>
          </div>
        </div>
      </main>

      {/* Create Key Modal */}
      {showCreateModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
          <div className="bg-white rounded-xl max-w-md w-full p-6">
            {!generatedKey ? (
              <>
                <h3 className="text-2xl font-bold text-gray-800 mb-4">
                  Create New API Key
                </h3>
                <p className="text-gray-600 mb-4">
                  Give your API key a descriptive name to help you identify it
                  later.
                </p>
                <input
                  type="text"
                  value={newKeyName}
                  onChange={(e) => setNewKeyName(e.target.value)}
                  placeholder="e.g., My QGIS Desktop"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent mb-4"
                />
                <div className="flex gap-2 justify-end">
                  <button
                    onClick={() => {
                      setShowCreateModal(false);
                      setNewKeyName("");
                    }}
                    className="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition"
                  >
                    Cancel
                  </button>
                  <button
                    onClick={handleCreateKey}
                    disabled={!newKeyName.trim() || createApiKeyMutation.isPending}
                    className="px-4 py-2 bg-gradient-to-r from-purple-500 to-indigo-600 text-white rounded-lg hover:from-purple-600 hover:to-indigo-700 transition font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {createApiKeyMutation.isPending ? "Creating..." : "Create Key"}
                  </button>
                </div>
              </>
            ) : (
              <>
                <div className="flex items-start gap-3 mb-4 p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
                  <AlertCircle className="text-yellow-600 flex-shrink-0 mt-0.5" size={20} />
                  <div className="text-sm text-yellow-800">
                    <p className="font-semibold mb-1">Save this key now!</p>
                    <p>
                      This is the only time you'll see this key. Make sure to
                      copy it somewhere safe.
                    </p>
                  </div>
                </div>
                <div className="mb-4">
                  <label className="block text-sm font-semibold text-gray-700 mb-2">
                    Your API Key
                  </label>
                  <div className="flex gap-2">
                    <input
                      type="text"
                      value={generatedKey}
                      readOnly
                      className="flex-1 px-4 py-2 bg-gray-50 border border-gray-300 rounded-lg font-mono text-sm"
                    />
                    <button
                      onClick={() => copyToClipboard(generatedKey)}
                      className="px-4 py-2 bg-gray-100 hover:bg-gray-200 rounded-lg transition"
                      title="Copy to clipboard"
                    >
                      <Copy size={20} />
                    </button>
                  </div>
                </div>
                <button
                  onClick={() => {
                    setShowCreateModal(false);
                    setGeneratedKey(null);
                    // TODO: Refresh API keys list
                  }}
                  className="w-full px-4 py-2 bg-gradient-to-r from-purple-500 to-indigo-600 text-white rounded-lg hover:from-purple-600 hover:to-indigo-700 transition font-semibold"
                >
                  Done
                </button>
              </>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
