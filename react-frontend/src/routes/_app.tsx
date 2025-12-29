import { App } from "@/features/app/components/App";
import { createFileRoute, Navigate } from "@tanstack/react-router";
import { useAuth } from "@clerk/clerk-react";

type MapParams = {
  projects?: string;
};

const AppLayout = () => {
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

  return <App />;
};

export const Route = createFileRoute("/_app")({
  validateSearch: (search: Record<string, unknown>): MapParams => {
    return {
      projects: search.projects as string,
    };
  },
  component: AppLayout,
});
