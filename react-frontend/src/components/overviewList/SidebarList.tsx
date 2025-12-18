import SidebarListBody from "./SidebarListBody";
import { useState } from "react";

import Project from "@/domain/project/entity";
import SortBy, { SORT_OPTIONS } from "./SortBy";

interface Props {
  items: Project[];
  // showArchived: boolean;
  // setShowArchived: React.Dispatch<React.SetStateAction<boolean>>;
}

export default function SidebarList({ items }: Props) {
  const [sortBy, setSortBy] = useState(SORT_OPTIONS.NAME_ASCENDING);

  return (
    <div className=" py-2">
      <div className=" flex items-center">
        {/* {newDataType && (
          <div className="pl-2">
            <AddDataButton newDataType={newDataType} />
          </div>
        )} */}
        {/* {showArchivedToggle && (
          <div className="ml-auto">{showArchivedToggle()}</div>
        )} */}
      </div>
      <div className="px-4 flex justify-between items-center">
        <SortBy sortBy={sortBy} setSortBy={setSortBy} />
      </div>
      <SidebarListBody sortBy={sortBy} items={items} />
    </div>
  );
}
