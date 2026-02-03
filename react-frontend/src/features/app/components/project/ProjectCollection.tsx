import type {
  ProjectCollectionItem,
  ProjectCollectionItems,
} from "@/domain/projectCollectionItems/outputDTO";

import type { ReactNode } from "react";
import UserInitials from "../UserInitials";
import SetPrimaryRadio from "./SetPrimaryRadio";

import { FeatureActionsDropdown } from "./features/FeatureActionsDropdown";

export const ProjectCollection = ({
  data,
  showArchived,
}: {
  data: ProjectCollectionItems;
  showArchived: boolean;
}) => {
  const features = showArchived
    ? data.features
    : data.features.filter((f) => f.properties.status !== "ARCHIVED");
  return (
    <table className="table table-fixed">
      <SiteDataTableHeadings>
        <></>
      </SiteDataTableHeadings>
      <tbody>
        {features.map((f) => (
          <SiteDataTableRow key={f.id} item={f}>
            <></>
          </SiteDataTableRow>
        ))}
      </tbody>
    </table>
  );
};

export function SiteDataTableHeadings({ children }: { children: ReactNode }) {
  return (
    <thead>
      <tr>
        <th className="w-12 p-0">Id</th>
        <th className="w-6 p-0 text-center"></th>
        <th className="p-0">Name</th>
        {children}
        <th className="w-11 p-0 text-center">Primary</th>
        <th className="w-8"></th>
        <th className="w-8"></th>
      </tr>
    </thead>
  );
}

export function SiteDataTableRow({
  children,
  item,
}: {
  children: ReactNode;
  item: ProjectCollectionItem;
}) {
  return (
    <tr key={item.id}>
      <td className="p-0">
        <span
          className={
            item.properties.status === "ARCHIVED" ? "text-base-content/50" : ""
          }
        >
          {item.id}
        </span>
      </td>
      <td className="p-0">
        <div className="flex">
          <input
            id={`c${item.properties.collection_id}item${item.id}`}
            name="show-ellipse-checkbox"
            type="checkbox"
            className="checkbox checkbox-sm bg-base-100"
          ></input>
        </div>
      </td>
      <td className="p-0 text-wrap break-words ">
        <label
          htmlFor={`c${item.properties.collection_id}item${item.id}`}
          className="cursor-pointer"
        >
          {" "}
          <span
            className={
              item.properties.status === "ARCHIVED"
                ? "text-base-content/50"
                : ""
            }
          >
            {item.properties.name}
          </span>
        </label>
      </td>
      {children}

      <td className="p-0">
        <div className="flex justify-center">
          <SetPrimaryRadio item={item} />
        </div>
      </td>
      <td className="p-0 text-center">
        <UserInitials
          firstName={item.properties.added_by_first_name}
          lastName={item.properties.added_by_last_name}
          message={`added by: ${item.properties.added_by_first_name} ${item.properties.added_by_last_name}`}
        />
      </td>
      <td className="px-0 py-2 text-right">
        <FeatureActionsDropdown item={item} />
      </td>
    </tr>
  );
}
