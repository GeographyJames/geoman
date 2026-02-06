import { SignedIn, UserButton } from "@clerk/clerk-react";
import { useState } from "react";

import ApiKeysSection from "./sections/apiKey/ApiKeysSection";
import TeamsSection from "./sections/TeamsSection";
import CollectionsSection from "./sections/collections/CollectionsSection";

type Tab = "teams" | "collections" | "api-keys";

export default function AdminPage() {
  const [activeTab, setActiveTab] = useState<Tab>("teams");

  return (
    <div className="min-h-screen bg-base-200">
      {/* Header */}
      <div className="navbar bg-base-100 border-b border-base-300">
        <div className="flex-1">
          <a href="/" className="btn btn-ghost text-xl font-bold">
            GeoMan
          </a>
          <div className="ml-4">
            <div className="tabs tabs-boxed bg-transparent">
              <a
                className={`tab ${activeTab === "teams" ? "tab-active" : ""}`}
                onClick={() => setActiveTab("teams")}
              >
                Teams
              </a>
              <a
                className={`tab ${activeTab === "collections" ? "tab-active" : ""}`}
                onClick={() => setActiveTab("collections")}
              >
                Collections
              </a>
              <a
                className={`tab ${activeTab === "api-keys" ? "tab-active" : ""}`}
                onClick={() => setActiveTab("api-keys")}
              >
                API Keys
              </a>
            </div>
          </div>
        </div>
        <div className="flex-none gap-2">
          {__RUN_ENVIRONMENT__ === "demo" ? (
            <div
              className="tooltip tooltip-left"
              data-tip="User authentication disabled in demo mode"
            >
              <div className="badge badge-warning">Demo</div>
            </div>
          ) : (
            <SignedIn>
              <UserButton />
            </SignedIn>
          )}
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-6xl mx-auto p-6">
        {activeTab === "teams" && <TeamsSection />}
        {activeTab === "collections" && <CollectionsSection />}
        {activeTab === "api-keys" && <ApiKeysSection />}
      </div>
    </div>
  );
}
