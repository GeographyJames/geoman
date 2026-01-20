import { useNavigate, useSearch } from "@tanstack/react-router";

import { CloseButton } from "@/components/Buttons";
import type Project from "@/domain/project/entity";
import { useProjectCollections } from "@/hooks/api/useProjectCollections";
import { SiteDataDropdown } from "./siteDataDropdown";
import { ProjectIcons } from "./ProjectIcons";
import { ProjectActionsDropdown } from "./ProjectActionsDropdown";

export const ProjectPanel = ({ project }: { project: Project }) => {
  const navigate = useNavigate();
  const search = useSearch({ from: "/_app/" });
  const { data: collectionsData, isLoading } = useProjectCollections({
    projectId: project.id,
  });

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
      className="collapse collapse-arrow bg-base-100 min-w-0 w-full rounded-box relative flex-shrink-0 shadow-lg"
      open={defaultExpanded}
    >
      <summary className="collapse-title after:start-5 after:end-auto ps-12 text-l font-bold pr-4 flex justify-between items-center py-2">
        {project.name}
        <div className="flex items-center gap-2 font-normal">
          <ProjectIcons project={project} />
          <ProjectActionsDropdown
            item={project}
            id={`{panel-p-${project.id}}`}
          />
          <CloseButton onClick={handleClose} />
        </div>
      </summary>

      <div className="collapse-content ">
        <div className="flex flex-col gap-2">
          {isLoading ? (
            <span className="loading loading-spinner loading-sm"></span>
          ) : collectionsData?.collections &&
            collectionsData.collections.length > 0 ? (
            collectionsData.collections.map((collection) => (
              <SiteDataDropdown
                collection={collection}
                projectId={project.id}
              />
            ))
          ) : (
            <p className="text-sm text-base-content/70">No collections found</p>
          )}
        </div>
      </div>
    </details>
  );
};
