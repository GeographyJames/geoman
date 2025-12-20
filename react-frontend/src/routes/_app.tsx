import { App } from "@/features/app/components/App";
import { createFileRoute } from "@tanstack/react-router";

const AppLayout = () => {
  return <App />;
};

export const Route = createFileRoute("/_app")({
  component: AppLayout,
});
