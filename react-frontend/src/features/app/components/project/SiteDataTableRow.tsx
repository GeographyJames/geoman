import { type ReactNode } from "react";
import { createPortal } from "react-dom";
import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";
import { type WakePreset } from "@/lib/turbineAreas";
import MapPopup from "@/components/mapComponents/MapPopup";
import UserInitials from "@/components/UserInitials";
import SetPrimaryRadio from "./SetPrimaryRadio";
import { FeatureActionsDropdown } from "./features/FeatureActionsDropdown";
import { dateFormat } from "@/constants";
import { TurbinePopupContent } from "./TurbinePopupContent";
import { useCollectionItemLayer } from "./useCollectionItemLayer";

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
  const { popupRef, popupContent, closePopup, zoomToFeature } = useCollectionItemLayer(item, {
    visible,
    areasVisible,
    wakePreset,
    windFromDeg,
    projectSlug,
    collectionSlug,
  });

  return (
    <tr key={item.id} className="hover:bg-base-200">
      {createPortal(
        <MapPopup ref={popupRef} onClose={closePopup}>
          {popupContent && <TurbinePopupContent turbine={popupContent} />}
        </MapPopup>,
        document.body,
      )}
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
