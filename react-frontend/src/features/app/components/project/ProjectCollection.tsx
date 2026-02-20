import type {
  ProjectCollectionItem,
  ProjectCollectionItems,
} from "@/domain/projectCollectionItems/outputDTO";

import { useState, type ReactNode } from "react";
import UserInitials from "../UserInitials";
import SetPrimaryRadio from "./SetPrimaryRadio";
import { useFeatureLayer, useZoomToFeature } from "@/hooks/useFeatureLayer";
import { Stroke, Fill, Style, Circle } from "ol/style";

import { FeatureActionsDropdown } from "./features/FeatureActionsDropdown";
import { dateFormat } from "@/constants";

const primaryStyle = new Style({
  stroke: new Stroke({
    color: "#DC2626",
    width: 2.5,
  }),
  fill: new Fill({
    color: "rgba(220, 38, 38, 0.12)",
  }),
  image: new Circle({
    radius: 6,
    fill: new Fill({ color: "#DC2626" }),
    stroke: new Stroke({ color: "#fff", width: 1.5 }),
  }),
});

const defaultStyle = new Style({
  stroke: new Stroke({
    color: "#2563EB",
    width: 2.5,
  }),
  fill: new Fill({
    color: "rgba(37, 99, 235, 0.12)",
  }),
  image: new Circle({
    radius: 6,
    fill: new Fill({ color: "#2563EB" }),
    stroke: new Stroke({ color: "#fff", width: 1.5 }),
  }),
});

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
        <th className="w-16 p-0">CRS ID</th>
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
  const [visible, setVisible] = useState(item.properties.is_primary);
  useFeatureLayer(visible ? item : undefined, item.properties.is_primary ? primaryStyle : defaultStyle);
  const zoomToFeature = useZoomToFeature(item);

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
            type="checkbox"
            className="checkbox checkbox-sm bg-base-100"
            checked={visible}
            onChange={(e) => setVisible(e.target.checked)}
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
            {item.properties.name}{" "}
            {item.properties.status === "ARCHIVED" && (
              <span className="text-xs">(archived)</span>
            )}
          </span>
        </label>
      </td>
      <td className="p-0 text-sm">{item.properties.storage_crs_srid}</td>
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
          message={`added by: ${item.properties.added_by_first_name} ${item.properties.added_by_last_name} ${dateFormat.format(new Date(item.properties.added))}`}
        />
      </td>
      <td className="px-0 py-2 text-right">
        <FeatureActionsDropdown item={item} zoomToFeature={zoomToFeature} />
      </td>
    </tr>
  );
}
