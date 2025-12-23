
import { FiUsers, FiLock, FiGlobe } from "react-icons/fi";


import type { IconType } from "react-icons";



export const Status = {
  Active: "ACTIVE",
  Archived: "ARCHIVED",
  Deleted: "DELETED"
} as const;
export type Status = typeof Status[keyof typeof Status];

export const Visibility = {
  Private: "PRIVATE",
  Team: "TEAM",
  Public: "PUBLIC"
} as const;
export type Visibility = typeof Visibility[keyof typeof Visibility];

  // Configuration for visibility icons
  export const VisibilityConfig: Record<Visibility, { icon: IconType; label: string; description: string }> = {
    [Visibility.Private]: {
      icon: FiLock,
      label: "Private",
      description: "only visible to you and other project members"
    },
    [Visibility.Team]: {
      icon: FiUsers,
      label: "Team",
      description: "visible to you, other project members, and your team"
    },
    [Visibility.Public]: {
      icon: FiGlobe,
      label: "Public",
      description: "visible to whole organisation"
    }
  };