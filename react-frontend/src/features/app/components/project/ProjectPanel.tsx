import { memo } from "react";
import { useNavigate, useSearch } from "@tanstack/react-router";

import { CloseButton } from "@/components/Buttons";
import type Project from "@/domain/project/entity";
import { useProjectCollections } from "@/hooks/api/useProjectCollections";
import { useProjectCollectionItems } from "@/hooks/api/useProjectCollectionItems";
import { useZoomToProjectBoundary } from "@/hooks/useZoomToProjectBoundary";
import { SiteDataDropdown } from "./siteDataDropdown";
import { ProjectIcons } from "./ProjectIcons";
import { ProjectActionsDropdown } from "./ProjectActionsDropdown";

const SITE_BOUNDARIES_TITLE = "site boundaries";

export const ProjectPanel = memo(({ project }: { project: Project }) => {
  const navigate = useNavigate();
  const search = useSearch({ from: "/_app/" });
  const { data: collectionsData, isLoading } = useProjectCollections({
    projectId: project.id,
  });

  const siteBoundariesCollection = collectionsData?.collections.find(
    (c) => c.title.toLowerCase() === SITE_BOUNDARIES_TITLE,
  );

  const { data: itemsData } = useProjectCollectionItems({
    projectId: project.id,
    collectionId: siteBoundariesCollection?.id ?? "",
    enabled: !!siteBoundariesCollection,
  });

  const primaryFeature = itemsData?.features.find(
    (f) => f.properties.is_primary,
  );

  const { zoomToProject, hasExtent } = useZoomToProjectBoundary(primaryFeature);

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
        <p>
          {project.name}{" "}
          <span className="text-sm font-normal text-base-content/70">
            <span className="hidden sm:inline">
              {project.outputDto.properties.crs_srid &&
                `EPSG:${project.outputDto.properties.crs_srid}`}
            </span>
            {project.archived && " (archived)"}
          </span>
        </p>
        <div className="flex items-center gap-2 font-normal">
          <ProjectIcons project={project} />
          <ProjectActionsDropdown
            item={project}
            id={`panel-p-${project.id}`}
            zoomToProject={zoomToProject}
            hasExtent={hasExtent}
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
                key={collection.id}
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
});
