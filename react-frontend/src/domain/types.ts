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