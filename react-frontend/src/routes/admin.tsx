import AdminPage from "@/features/admin/AdminPage";
import { useAuth } from "@clerk/clerk-react";
import { createFileRoute, Navigate } from "@tanstack/react-router";

export const Route = createFileRoute("/admin")({
  component: Admin,
});

export default function Admin() {
  const { isSignedIn, isLoaded } = useAuth();
  const isDemoMode = __RUN_ENVIRONMENT__ === "development";

  // Wait for Clerk to load
  if (!isLoaded) {
    return null;
  }

  // Redirect if not authenticated (unless demo mode)
  if (!isSignedIn && !isDemoMode) {
    return <Navigate to="/about" />;
  }

  return <AdminPage />;
}
