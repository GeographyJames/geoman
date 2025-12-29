import { createFileRoute } from "@tanstack/react-router";

import { ProjectAdmin } from "@/features/app/components/project/ProjectAdmin";

const ProjectRoute = () => {
  return <ProjectAdmin />;
};

export const Route = createFileRoute("/_app/project/$slug")({
  component: ProjectRoute,
});
