import ProjectsMap from "@/features/projects/components/ProjectsMap";
import { useProjects } from "@/hooks/api/useProjects";
import { createFileRoute } from "@tanstack/react-router";
import { useState } from "react";

export const Route = createFileRoute("/_app/")({
  component: ProjectsRoute,
});

function ProjectsRoute() {
  const [showArchived, setShowArchived] = useState<boolean>(false);
  const { data } = useProjects();
  if (data) {
    return <ProjectsMap showArchived={showArchived} />;
  }
}
