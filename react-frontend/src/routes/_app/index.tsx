import ProjectsMap from "@/features/projects/components/ProjectsMap";
import { useProjects } from "@/hooks/api/useProjects";
import { createFileRoute } from "@tanstack/react-router";

const ProjectsRoute = () => {
  const { data } = useProjects();
  if (data) {
    return <ProjectsMap />;
  }
};

export const Route = createFileRoute("/_app/")({
  component: ProjectsRoute,
});
