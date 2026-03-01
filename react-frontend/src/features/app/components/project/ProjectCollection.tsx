import type {
  ProjectCollectionItem,
  ProjectCollectionItems,
} from "@/domain/projectCollectionItems/outputDTO";

import {
  type Dispatch,
  type ReactNode,
  type SetStateAction,
  useMemo,
  useState,
} from "react";
import UserInitials from "@/components/UserInitials";
import SetPrimaryRadio from "./SetPrimaryRadio";
import {
  useFeatureCollectionLayer,
  useFeatureLayer,
  useZoomToFeature,
} from "@/hooks/useFeatureLayer";
import { Stroke, Fill, Style, Circle } from "ol/style";
import { type WakePreset, generateTurbineAreas } from "@/lib/turbineAreas";

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

const sweptAreaStyle = new Style({
  stroke: new Stroke({ color: "rgba(37, 99, 235, 0.5)", width: 1 }),
  fill: new Fill({ color: "rgba(37, 99, 235, 0.06)" }),
});

const wakeEllipseStyle = new Style({
  stroke: new Stroke({
    color: "rgba(217, 119, 6, 0.6)",
    width: 1,
    lineDash: [4, 4],
  }),
  fill: new Fill({ color: "rgba(217, 119, 6, 0.04)" }),
});

function formatArea(m2: number): string {
  return `${(m2 / 10_000).toLocaleString(undefined, { maximumFractionDigits: 1 })}`;
}

function formatTurbineMeasurement(
  value: number | "various" | null | undefined,
): string {
  if (value == null) return "none specified";
  if (value === "various") return "Various";
  return `${(value / 1000).toLocaleString()}m`;
}

export const ProjectCollection = ({
  data,
  showArchived,
  visibilityMap,
  setVisibilityMap,
  projectSlug,
  collectionSlug,
}: {
  data: ProjectCollectionItems;
  showArchived: boolean;
  visibilityMap: Record<number, boolean>;
  setVisibilityMap: Dispatch<SetStateAction<Record<number, boolean>>>;
  projectSlug: string;
  collectionSlug: string;
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

  const [showAreasMap, setShowAreasMap] = useState<Record<number, boolean>>({});
  const [wakePreset, setWakePreset] = useState<WakePreset>("6x4");
  const [windFromDeg, setWindFromDeg] = useState(225);

  return (
    <>
      {isTurbineLayout && (
        <div className="flex flex-wrap items-center gap-x-4 gap-y-1 pb-1 text-xs">
          <div className="flex items-center gap-2">
            <span className="text-base-content/70">
              Ellipse size (rotor diameters):
            </span>
            <div className="join">
              <input
                className="join-item btn btn-xs"
                type="radio"
                name={`wake-preset-${projectSlug}-${collectionSlug}`}
                aria-label="6×4"
                checked={wakePreset === "6x4"}
                onChange={() => setWakePreset("6x4")}
              />
              <input
                className="join-item btn btn-xs"
                type="radio"
                name={`wake-preset-${projectSlug}-${collectionSlug}`}
                aria-label="5×3"
                checked={wakePreset === "5x3"}
                onChange={() => setWakePreset("5x3")}
              />
            </div>
          </div>
          <div className="flex items-center gap-1">
            <span className="text-base-content/70">Wind direction:</span>
            <input
              type="range"
              className="range range-xs w-24"
              min={0}
              max={359}
              value={windFromDeg}
              onChange={(e) => setWindFromDeg(Number(e.target.value))}
            />
            <span className="text-base-content/70 w-8">{windFromDeg}°</span>
          </div>
        </div>
      )}
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
              <th className="w-12 p-0 hidden sm:table-cell text-wrap">
                Hub height
              </th>
              <th className="w-14 p-0 text-wrap text-center">Ellipses</th>
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
              areasVisible={showAreasMap[f.id] ?? true}
              wakePreset={wakePreset}
              windFromDeg={windFromDeg}
              projectSlug={projectSlug}
              collectionSlug={collectionSlug}
            >
              {hasArea && (
                <td className="p-0 hidden sm:table-cell">
                  <span
                    className={
                      f.properties.status === "ARCHIVED"
                        ? "text-base-content/50"
                        : ""
                    }
                  >
                    {f.properties.area_ellipsoidal_m2 != null
                      ? formatArea(f.properties.area_ellipsoidal_m2)
                      : ""}
                  </span>
                </td>
              )}
              {isTurbineLayout && (
                <>
                  <td className=" hidden sm:table-cell landscape:table-cell p-0 pr-2">
                    <span
                      className={
                        f.properties.status === "ARCHIVED"
                          ? "text-base-content/50"
                          : ""
                      }
                    >
                      {f.properties.turbine_count ?? ""}
                    </span>
                  </td>
                  <td className="p-0 hidden sm:table-cell">
                    <span
                      className={
                        f.properties.status === "ARCHIVED"
                          ? "text-base-content/50"
                          : ""
                      }
                    >
                      {formatTurbineMeasurement(f.properties.rotor_diameter_mm)}
                    </span>
                  </td>
                  <td className="flex p-0 hidden sm:table-cell">
                    <span
                      className={
                        f.properties.status === "ARCHIVED"
                          ? "text-base-content/50"
                          : ""
                      }
                    >
                      {formatTurbineMeasurement(f.properties.hub_height_mm)}
                    </span>
                  </td>
                  <td className="">
                    <div className="flex justify-center">
                      <input
                        type="checkbox"
                        className="checkbox checkbox-xs bg-base-100 "
                        checked={showAreasMap[f.id] ?? true}
                        disabled={!(visibilityMap[f.id] ?? false)}
                        onChange={(e) =>
                          setShowAreasMap((prev) => ({
                            ...prev,
                            [f.id]: e.target.checked,
                          }))
                        }
                      />
                    </div>
                  </td>
                </>
              )}
            </SiteDataTableRow>
          ))}
        </tbody>
      </table>
    </>
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
  areasVisible = true,
  wakePreset = "6x4",
  windFromDeg = 225,
  projectSlug,
  collectionSlug,
}: {
  children: ReactNode;
  item: ProjectCollectionItem;
  visible: boolean;
  onVisibleChange: (val: boolean) => void;
  areasVisible?: boolean;
  wakePreset?: WakePreset;
  windFromDeg?: number;
  projectSlug: string;
  collectionSlug: string;
}) {
  useFeatureLayer(
    visible ? item : undefined,
    item.properties.is_primary ? primaryStyle : defaultStyle,
  );
  const turbineAreas = useMemo(
    () =>
      visible && areasVisible
        ? generateTurbineAreas(item, wakePreset, windFromDeg)
        : null,
    [visible, areasVisible, wakePreset, windFromDeg, item],
  );
  useFeatureCollectionLayer(turbineAreas?.sweptAreas, sweptAreaStyle);
  useFeatureCollectionLayer(turbineAreas?.wakeEllipses, wakeEllipseStyle);
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
        <FeatureActionsDropdown
          item={item}
          zoomToFeature={zoomToFeature}
          projectSlug={projectSlug}
          collectionSlug={collectionSlug}
        />
      </td>
    </tr>
  );
}
