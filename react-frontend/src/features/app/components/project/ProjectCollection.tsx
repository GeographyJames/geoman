import type { ProjectCollectionItems } from "@/domain/projectCollectionItems/outputDTO";
import { type Dispatch, type SetStateAction, useState } from "react";
import { type WakePreset } from "@/lib/turbineAreas";
import { TURBINE_LAYOUTS_COLLECTION_ID } from "@/constants";
import { TurbineLayoutControls } from "./TurbineLayoutControls";
import { SiteDataTableHeadings, SiteDataTableRow } from "./SiteDataTableRow";

function formatArea(m2: number): string {
  return `${(m2 / 10_000).toLocaleString(undefined, { maximumFractionDigits: 1 })}ha`;
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
    TURBINE_LAYOUTS_COLLECTION_ID;

  const [showAreasMap, setShowAreasMap] = useState<Record<number, boolean>>({});
  const [showTurbineNumbersMap, setShowTurbineNumbersMap] = useState<
    Record<number, boolean>
  >({});
  const [wakePreset, setWakePreset] = useState<WakePreset>("6x4");
  const [windFromDeg, setWindFromDeg] = useState(225);

  return (
    <>
      {isTurbineLayout && (
        <TurbineLayoutControls
          name={`wake-preset-${projectSlug}-${collectionSlug}`}
          wakePreset={wakePreset}
          setWakePreset={setWakePreset}
          windFromDeg={windFromDeg}
          setWindFromDeg={setWindFromDeg}
        />
      )}
      <table className="table table-fixed table-xs">
        <SiteDataTableHeadings>
          {hasArea && <th className="w-18 p-0 hidden sm:table-cell">Area</th>}
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
              <th className="w-14 p-0 text-wrap text-center">
                Turbine
                <br />
                numbers
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
              areasVisible={
                showAreasMap[f.id] ?? f.properties.rotor_diameter_mm != null
              }
              wakePreset={wakePreset}
              windFromDeg={windFromDeg}
              showTurbineNumbers={showTurbineNumbersMap[f.id] ?? true}
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
                        className="checkbox checkbox-xs bg-base-100"
                        checked={showTurbineNumbersMap[f.id] ?? true}
                        disabled={!(visibilityMap[f.id] ?? false)}
                        onChange={(e) =>
                          setShowTurbineNumbersMap((prev) => ({
                            ...prev,
                            [f.id]: e.target.checked,
                          }))
                        }
                      />
                    </div>
                  </td>
                  <td className="">
                    <div className="flex justify-center">
                      <input
                        type="checkbox"
                        className="checkbox checkbox-xs bg-base-100 "
                        checked={
                          showAreasMap[f.id] ??
                          f.properties.rotor_diameter_mm != null
                        }
                        disabled={
                          !(visibilityMap[f.id] ?? false) ||
                          f.properties.rotor_diameter_mm == null
                        }
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
