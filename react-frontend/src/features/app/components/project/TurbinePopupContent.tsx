import type { TurbinePopupData } from "./useTurbineLayerWithPopup";

function formatMm(mm: number | null): string {
  if (mm == null) return "none specified";
  return `${(mm / 1000).toLocaleString()}m`;
}

export function TurbinePopupContent({ turbine }: { turbine: TurbinePopupData }) {
  const [lon, lat] = turbine.geometry.coordinates;
  const crsLabel = turbine.storage_crs_name
    ? `${turbine.storage_crs_name} (EPSG:${turbine.storage_crs_srid})`
    : `EPSG:${turbine.storage_crs_srid}`;
  return (
    <div className="space-y-0.5">
      <p className="font-semibold">Turbine {turbine.properties.turbine_number}</p>
      <p>Hub height: {formatMm(turbine.properties.hub_height_mm)}</p>
      <p>Rotor diameter: {formatMm(turbine.properties.rotor_diameter_mm)}</p>
      <p>Lat: {lat.toFixed(6)}</p>
      <p>Long: {lon.toFixed(6)}</p>
      <p className="font-semibold mt-1">{crsLabel}</p>
      <p>x: {turbine.properties.x_storage_crs}</p>
      <p>y: {turbine.properties.y_storage_crs}</p>
    </div>
  );
}
