import AdminPage from "@/features/AdminPage";
import { useCurrentUser } from "@/hooks/api/useCurrentUser";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/admin")({
  component: Admin,
});

export default function Admin() {
  const { data } = useCurrentUser();
  if (!data) {
    return <>Error loading user</>;
  }
  return <AdminPage currentUser={data} />;
}
