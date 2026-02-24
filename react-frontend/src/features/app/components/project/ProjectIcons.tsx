import { memo } from "react";
import type Project from "@/domain/project/entity";
import { VisibilityConfig } from "@/domain/types";
import UserInitials from "@/components/UserInitials";
import { FaLocationDot } from "react-icons/fa6";
import { dateFormat } from "@/constants";

export const ProjectIcons = memo(function ProjectIcons({ project }: { project: Project }) {
  const vis = VisibilityConfig[project.visibility];
  const VisibilityIcon = vis.icon;
  return (
    <div className="flex">
      <div className="flex w-6  items-center justify-center">
        {project.centroid && <FaLocationDot size={18} />}
      </div>
      <div className="flex w-6 items-center justify-center">
        <div
          className="tooltip tooltip-left"
          data-tip={`${vis.label}: ${vis.description}`}
        >
          <VisibilityIcon size={20} />
        </div>
      </div>
      <div className="flex w-8 items-center justify-center">
        <UserInitials
          message={`project owner: ${project.ownerFirstName} ${project.ownerLastName}, added: ${dateFormat.format(project.added)}`}
          firstName={project.ownerFirstName}
          lastName={project.ownerLastName}
        />
      </div>
    </div>
  );
});
