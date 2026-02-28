import type {
  ProjectCollectionItem,
  ProjectCollectionItems,
} from "@/domain/projectCollectionItems/outputDTO";

import { type Dispatch, type ReactNode, type SetStateAction } from "react";
import UserInitials from "@/components/UserInitials";
import SetPrimaryRadio from "./SetPrimaryRadio";
import { useFeatureLayer, useZoomToFeature } from "@/hooks/useFeatureLayer";
import { Stroke, Fill, Style, Circle } from "ol/style";

import { FeatureActionsDropdown } from "./features/FeatureActionsDropdown";
import { dateFormat, TURBINE_LAYOUT_CCOLLECTION_ID } from "@/constants";

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

function formatArea(m2: number): string {
  return `${(m2 / 10_000).toLocaleString(undefined, { maximumFractionDigits: 1 })}`;
}

export const ProjectCollection = ({
  data,
  showArchived,
  visibilityMap,
  setVisibilityMap,
}: {
  data: ProjectCollectionItems;
  showArchived: boolean;
  visibilityMap: Record<number, boolean>;
  setVisibilityMap: Dispatch<SetStateAction<Record<number, boolean>>>;
}) => {
  const features = showArchived
    ? data.features
    : data.features.filter((f) => f.properties.status !== "ARCHIVED");

  if (features.length === 0) {
    return <p className="text-base-content/50 py-2 text-center">No features</p>;
  }

  const hasArea = features.some(
    (f) => f.properties.area_ellipsoidal_m2 != null,
  );

  const isTurbineLayout =
    data.features[0]?.properties.collection_id ===
    TURBINE_LAYOUT_CCOLLECTION_ID;

  return (
    <table className="table table-fixed table-xs">
      <SiteDataTableHeadings>
        {hasArea && (
          <th className="w-18 p-0 hidden sm:table-cell">Area (ha)</th>
        )}
        {isTurbineLayout && (
          <>
            <th className="hidden sm:table-cell landscape:table-cell w-14 p-0">
              No. of
              <br />
              turbines
            </th>
            <th className="w-14 p-0 hidden sm:table-cell text-wrap">
              Rotor diameter
            </th>
            <th className="w-14 p-0 hidden sm:table-cell text-wrap">
              Hub height
            </th>
          </>
        )}
      </SiteDataTableHeadings>
      <tbody>
        {features.map((f) => (
          <SiteDataTableRow
            key={f.id}
            item={f}
            visible={visibilityMap[f.id] ?? false}
            onVisibleChange={(val) =>
              setVisibilityMap((prev) => ({ ...prev, [f.id]: val }))
            }
          >
            {hasArea && (
              <td className="p-0 hidden sm:table-cell">
                {f.properties.area_ellipsoidal_m2 != null
                  ? formatArea(f.properties.area_ellipsoidal_m2)
                  : ""}
              </td>
            )}
            {isTurbineLayout && (
              <>
                <td className=" hidden sm:table-cell landscape:table-cell p-0 pr-2">
                  {f.properties.turbine_count ?? ""}
                </td>
                <td className="p-0 hidden sm:table-cell">
                  {f.properties.rotor_diameter_mm != null
                    ? `${(f.properties.rotor_diameter_mm / 1000).toLocaleString()}m`
                    : ""}
                </td>
                <td className="p-0 hidden sm:table-cell">
                  {f.properties.hub_height_mm != null
                    ? `${(f.properties.hub_height_mm / 1000).toLocaleString()}m`
                    : ""}
                </td>
              </>
            )}
          </SiteDataTableRow>
        ))}
      </tbody>
    </table>
  );
};

export function SiteDataTableHeadings({ children }: { children: ReactNode }) {
  return (
    <thead className="text-xs">
      <tr>
        <th className="w-12 p-0">Id</th>
        <th className="w-6 p-0"></th>
        <th className="p-0">Name</th>
        {children}
        <th className="w-16 p-0 hidden sm:table-cell">CRS ID</th>

        <th className="w-12 p-0 text-center">Primary</th>
        <th className="w-8 hidden sm:table-cell"></th>
        <th className="w-8"></th>
      </tr>
    </thead>
  );
}

export function SiteDataTableRow({
  children,
  item,
  visible,
  onVisibleChange,
}: {
  children: ReactNode;
  item: ProjectCollectionItem;
  visible: boolean;
  onVisibleChange: (val: boolean) => void;
}) {
  useFeatureLayer(
    visible ? item : undefined,
    item.properties.is_primary ? primaryStyle : defaultStyle,
  );
  const zoomToFeature = useZoomToFeature(item);

  return (
    <tr key={item.id} className="hover:bg-base-200">
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
            className="checkbox checkbox-xs bg-base-100"
            checked={visible}
            onChange={(e) => onVisibleChange(e.target.checked)}
          ></input>
        </div>
      </td>
      <td className="p-0 text-wrap break-words pr-2">
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

      {children}
      <td
        className={`p-0  hidden sm:table-cell ${item.properties.status === "ARCHIVED" ? "text-base-content/50" : ""}`}
      >
        {`EPSG:${item.properties.storage_crs_srid}`}
      </td>

      <td className="p-0">
        <div className="flex justify-center">
          <SetPrimaryRadio item={item} />
        </div>
      </td>
      <td className="p-0 text-center hidden sm:table-cell">
        <UserInitials
          firstName={item.properties.added_by_first_name}
          lastName={item.properties.added_by_last_name}
          message={`added by: ${item.properties.added_by_first_name} ${item.properties.added_by_last_name} ${dateFormat.format(new Date(item.properties.added))}`}
        />
      </td>
      <td className="px-0 py-1 text-right">
        <FeatureActionsDropdown item={item} zoomToFeature={zoomToFeature} />
      </td>
    </tr>
  );
}
