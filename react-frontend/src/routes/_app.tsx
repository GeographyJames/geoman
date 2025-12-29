import { App } from "@/features/app/components/App";
import { createFileRoute } from "@tanstack/react-router";

type MapParams = {
  projects?: string[];
};

const AppLayout = () => {
  return <App />;
};

export const Route = createFileRoute("/_app")({
  validateSearch: (search: Record<string, unknown>): MapParams => {
    return {
      projects: search.projects as string[],
    };
  },
  component: AppLayout,
});
