import AdminPage from "@/components/AdminPage";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/admin")({
  component: AdminPage,
});
