import ProjectsMap from "@/features/projects/components/ProjectsMap";
import { useProjects } from "@/hooks/api/useProjects";
import { createFileRoute } from "@tanstack/react-router";
import { useState } from "react";

const ProjectsRoute = () => {
  const [showArchived, setShowArchived] = useState<boolean>(false);
  const { data } = useProjects();
  if (data) {
    return <ProjectsMap showArchived={showArchived} />;
  }
};

export const Route = createFileRoute("/_app/")({
  component: ProjectsRoute,
});
