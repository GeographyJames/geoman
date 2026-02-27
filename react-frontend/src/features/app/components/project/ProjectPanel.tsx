import { memo } from "react";
import { useNavigate, useSearch } from "@tanstack/react-router";
import { Search } from "lucide-react";

import { CloseButton } from "@/components/Buttons";
import type Project from "@/domain/project/entity";
import { useProjectCollections } from "@/hooks/api/useProjectCollections";
import { useProjectCollectionItems } from "@/hooks/api/useProjectCollectionItems";
import { useZoomToProjectBoundary } from "@/hooks/useZoomToProjectBoundary";
import { SiteDataDropdown } from "./siteDataDropdown";
import { ProjectIcons } from "./ProjectIcons";
import { ProjectActionsDropdown } from "./ProjectActionsDropdown";
import { SITE_BOUNDARIES_COLLECTION_ID } from "@/constants";

export const ProjectPanel = memo(({ project }: { project: Project }) => {
  const navigate = useNavigate();
  const search = useSearch({ from: "/_app/" });
  const { data: collectionsData, isLoading } = useProjectCollections({
    projectId: project.id,
  });

  const siteBoundariesCollection = collectionsData?.collections.find(
    (c) => c.id === SITE_BOUNDARIES_COLLECTION_ID.toString(),
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
      <summary className="collapse-title after:start-5 after:end-auto ps-12 text-l font-bold pr-4 flex justify-between items-center py-2 shadow-md">
        <p className="flex items-start gap-2">
          <span className="text-sm font-normal text-base-content/70 min-w-6 self-center">
            {project.id}
          </span>
          <span>{project.name}</span>
          {project.archived && (
            <span className="text-sm font-normal text-base-content/70">
              (archived)
            </span>
          )}
        </p>
        <div className="flex items-center gap-1 font-normal">
          {project.outputDto.properties.crs_srid && (
            <span className="hidden sm:inline text-sm text-base-content/70">
              EPSG:{project.outputDto.properties.crs_srid}
            </span>
          )}
          <ProjectIcons project={project} />
          <ProjectActionsDropdown item={project} id={`panel-p-${project.id}`} />
          <button
            className="btn btn-ghost btn-circle btn-sm disabled:opacity-40"
            title="Zoom to project"
            onClick={zoomToProject}
            disabled={!hasExtent}
          >
            <Search size={16} />
          </button>
          <CloseButton onClick={handleClose} />
        </div>
      </summary>

      <div className="collapse-content p-0 ">
        <div className="tabs tabs-box bg-base-300 p-2 rounded-none">
          <input
            type="radio"
            name={`tabs_${project.id}`}
            className="tab"
            aria-label="Collections"
            defaultChecked
          />
          <div className="tab-content bg-base-100  bg-base-300 p-0">
            <div className="flex flex-col gap-1">
              {isLoading ? (
                <span className="loading loading-spinner loading-sm"></span>
              ) : collectionsData?.collections &&
                collectionsData.collections.length > 0 ? (
                collectionsData.collections.map((collection) => (
                  <SiteDataDropdown
                    key={collection.id}
                    collection={collection}
                    project={project}
                  />
                ))
              ) : (
                <div className=" bg-base-100 border-base-300 p-6 rounded-sm shadow-lg">
                  <p className="text-sm text-base-content/70">
                    No collections found
                  </p>
                </div>
              )}
            </div>
          </div>

          <input
            type="radio"
            name={`tabs_${project.id}`}
            className="tab"
            aria-label="Figures"
          />
          <div className="tab-content bg-base-100 border-base-300 p-6 rounded-sm shadow-lg">
            Figures go here
          </div>

          <input
            type="radio"
            name={`tabs_${project.id}`}
            className="tab"
            aria-label="Information"
          />
          <div className="tab-content bg-base-100 border-base-300 p-6 rounded-sm shadow-lg">
            Information goes here
          </div>
          <input
            type="radio"
            name={`tabs_${project.id}`}
            className="tab"
            aria-label="Project members"
          />
          <div className="tab-content bg-base-100 border-base-300 p-6 rounded-sm shadow-lg">
            Project members goes here
          </div>
        </div>
      </div>
    </details>
  );
});
