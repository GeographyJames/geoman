import { createFileRoute } from "@tanstack/react-router";

import { ProjectPanel } from "@/features/app/components/project/ProjectPanel";

const ProjectRoute = () => {
  return <ProjectPanel />;
};

export const Route = createFileRoute("/_app/project/$slug")({
  component: ProjectRoute,
});
