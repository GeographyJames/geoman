import {
  SignedIn,
  SignedOut,
  SignInButton,
  UserButton,
} from "@clerk/clerk-react";
import { Link } from "@tanstack/react-router";
import { Map, Database, Globe } from "lucide-react";

export default function LandingPage() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-600 to-indigo-700">
      <header className="p-6">
        <div className="max-w-7xl mx-auto flex justify-between items-center">
          <Link to="/" className="flex items-center gap-2 text-white">
            <Map size={32} />
            <h1 className="text-2xl font-bold">GeoMan</h1>
          </Link>
          {__RUN_ENVIRONMENT__ === "demo" ? (
            <div
              className="tooltip tooltip-left"
              data-tip="User authentication disabled in demo mode"
            >
              <div className="badge badge-warning">Demo</div>
            </div>
          ) : (
            <>
              <SignedIn>
                <UserButton />
              </SignedIn>
              <SignedOut>
                <SignInButton mode="modal">
                  <button
                    type="button"
                    className="px-4 py-2 bg-white text-purple-600 rounded-lg font-semibold hover:bg-gray-100 transition"
                  >
                    Sign In
                  </button>
                </SignInButton>
              </SignedOut>
            </>
          )}
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-6 py-16">
        <div className="text-center mb-16">
          <h2 className="text-5xl font-bold text-white mb-4">
            Geospatial Data Management Platform
          </h2>
          <p className="text-xl text-purple-100 max-w-2xl mx-auto">
            Standards-compliant OGC API Features with powerful PostGIS
            integration. Manage your spatial data with ease.
          </p>
        </div>

        <div className="grid md:grid-cols-3 gap-8 mb-16">
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-8 text-white">
            <div className="w-12 h-12 bg-purple-500 rounded-lg flex items-center justify-center mb-4">
              <Globe size={24} />
            </div>
            <h3 className="text-xl font-bold mb-2">OGC API Features</h3>
            <p className="text-purple-100">
              Standards-compliant geospatial API supporting multiple coordinate
              reference systems and filtering.
            </p>
          </div>

          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-8 text-white">
            <div className="w-12 h-12 bg-purple-500 rounded-lg flex items-center justify-center mb-4">
              <Database size={24} />
            </div>
            <h3 className="text-xl font-bold mb-2">PostGIS Integration</h3>
            <p className="text-purple-100">
              Powerful spatial database capabilities with support for complex
              geometries and spatial operations.
            </p>
          </div>

          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-8 text-white">
            <div className="w-12 h-12 bg-purple-500 rounded-lg flex items-center justify-center mb-4">
              <Map size={24} />
            </div>
            <h3 className="text-xl font-bold mb-2">QGIS Compatible</h3>
            <p className="text-purple-100">
              Seamlessly integrate with QGIS and other GIS clients via OGC
              standards.
            </p>
          </div>
        </div>

        <SignedIn>
          <div className="bg-white rounded-xl p-8 shadow-2xl mb-8">
            <h3 className="text-2xl font-bold mb-4 text-gray-800">
              Welcome back!
            </h3>
            <p className="text-gray-600 mb-6">
              Manage your API keys and access your geospatial data.
            </p>
            <a
              href="/admin"
              className="inline-block px-6 py-3 bg-gradient-to-r from-purple-500 to-indigo-600 text-white rounded-lg hover:from-purple-600 hover:to-indigo-700 transition font-semibold text-lg"
            >
              Go to Admin Dashboard
            </a>
          </div>
        </SignedIn>

        <div className="bg-white rounded-xl p-8 shadow-2xl">
          <h3 className="text-2xl font-bold mb-6 text-gray-800">Quick Links</h3>
          <div className="grid md:grid-cols-2 gap-4">
            <a
              href="/ogcapi"
              className="block p-4 bg-gradient-to-r from-purple-500 to-indigo-600 text-white rounded-lg hover:from-purple-600 hover:to-indigo-700 transition"
            >
              <h4 className="font-bold mb-1">OGC API Features</h4>
              <p className="text-sm text-purple-100">Access the landing page</p>
            </a>
            <a
              href="/ogcapi/conformance"
              className="block p-4 bg-gradient-to-r from-indigo-500 to-purple-600 text-white rounded-lg hover:from-indigo-600 hover:to-purple-700 transition"
            >
              <h4 className="font-bold mb-1">Conformance Declaration</h4>
              <p className="text-sm text-purple-100">
                View API conformance classes
              </p>
            </a>
            <a
              href="/ogcapi/collections"
              className="block p-4 bg-gradient-to-r from-purple-500 to-pink-600 text-white rounded-lg hover:from-purple-600 hover:to-pink-700 transition"
            >
              <h4 className="font-bold mb-1">Collections</h4>
              <p className="text-sm text-purple-100">
                Browse feature collections
              </p>
            </a>
            <a
              href="/docs"
              className="block p-4 bg-gradient-to-r from-pink-500 to-purple-600 text-white rounded-lg hover:from-pink-600 hover:to-purple-700 transition"
            >
              <h4 className="font-bold mb-1">API Documentation</h4>
              <p className="text-sm text-purple-100">
                Interactive API reference
              </p>
            </a>
            <a
              href="/book"
              className="block p-4 bg-gradient-to-r from-indigo-500 to-blue-600 text-white rounded-lg hover:from-indigo-600 hover:to-blue-700 transition"
            >
              <h4 className="font-bold mb-1">Documentation</h4>
              <p className="text-sm text-purple-100">
                User guide and tutorials
              </p>
            </a>
          </div>
        </div>
      </main>

      <footer className="text-center py-8 text-purple-200">
        <p>Powered by Rust 🦀 | PostGIS | Actix Web</p>
      </footer>
    </div>
  );
}
