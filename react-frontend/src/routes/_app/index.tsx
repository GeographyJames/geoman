import { App } from "@/features/app/components/App";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_app/")({
  component: App,
});
