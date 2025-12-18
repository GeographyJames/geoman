import Project from "@/domain/project/entity";

import SidebarListItem from "./SidebarListItem";

interface Props {
  sortBy: string;
  items: Project[];
}

export default function SidebarListBody({ items, sortBy }: Props) {
  const sortedData = items.slice().sort((a, b) => {
    switch (sortBy) {
      case "name_ascending":
        return a.name.localeCompare(b.name);
      case "created":
        return b.added.getTime() - a.added.getTime();
      case "id_ascending":
        return a.id - b.id;
      case "id_descending":
        return b.id - a.id;
      default:
        return 0;
    }
  });

  return (
    <ul className="menu  w-full">
      {sortedData.map((item) => (
        <SidebarListItem key={item.id} item={item} />
      ))}
    </ul>
  );
}
