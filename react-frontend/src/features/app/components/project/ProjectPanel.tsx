import { useNavigate, useSearch } from "@tanstack/react-router";

import { CloseButton } from "@/components/Buttons";
import type Project from "@/domain/project/entity";

export const ProjectPanel = ({ project }: { project: Project }) => {
  const navigate = useNavigate();
  const search = useSearch({ from: "/_app/" });

  // Default to expanded on medium screens and up (sm breakpoint is 640px)
  const defaultExpanded = window.matchMedia("(min-width: 640px)").matches;

  const handleClose = () => {
    const currentProjects = search.projects || "";
    const projectsArray = currentProjects.split(",").filter(Boolean);
    const updatedProjects = projectsArray
      .filter((p) => p !== project.slug)
      .join(",");

    navigate({
      from: "/",
      search: { ...search, projects: updatedProjects || undefined },
    });
  };

  return (
    <details
      className="collapse collapse-arrow  bg-base-100 min-w-0 w-full max-w-[600px] shadow-lg rounded-box relative"
      open={defaultExpanded}
    >
      <summary className="collapse-title after:start-5 after:end-auto ps-12 text-l font-bold pr-4 flex justify-between items-center py-2">
        {project.name}
        <div className="">
          <CloseButton onClick={handleClose} />
        </div>
      </summary>

      <div className="collapse-content">
        <div className="flex h-96">
          <p>Project details will go here</p>
        </div>
      </div>
    </details>
  );
};
