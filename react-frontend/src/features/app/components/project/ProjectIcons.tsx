import type Project from "@/domain/project/entity";
import { VisibilityConfig } from "@/domain/types";
import { GiWindTurbine } from "react-icons/gi";
import UserInitials from "../UserInitials";
import { FaLocationDot } from "react-icons/fa6";

export function ProjectIcons({ project }: { project: Project }) {
  const vis = VisibilityConfig[project.visibility];
  const VisibilityIcon = vis.icon;
  return (
    <div className="flex">
      <div className="flex w-6 items-center items-center justify-center">
        {project.hasWind && project.primaryLayoutTurbineCount && (
          <div
            className="tooltip tooltip-left"
            data-tip={`${project.primaryLayoutTurbineCount} turbines`}
          >
            <GiWindTurbine size={20} />
          </div>
        )}
      </div>
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
          message={`project owner: ${project.ownerFirstName} ${project.ownerLastName}`}
          firstName={project.ownerFirstName}
          lastName={project.ownerLastName}
        />
      </div>
    </div>
  );
}
