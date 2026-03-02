const EARTH_RADIUS_M = 6371000;

function generateEllipse(
  lon: number,
  lat: number,
  semiMajorM: number,
  semiMinorM: number,
  bearingRad: number,
  nPoints = 64,
): GeoJSON.Position[] {
  const latRad = (lat * Math.PI) / 180;
  const coords: GeoJSON.Position[] = [];
  for (let i = 0; i <= nPoints; i++) {
    const theta = (2 * Math.PI * i) / nPoints;
    const xAlong = semiMajorM * Math.cos(theta);
    const xPerp = semiMinorM * Math.sin(theta);
    const east = xAlong * Math.sin(bearingRad) + xPerp * Math.cos(bearingRad);
    const north = xAlong * Math.cos(bearingRad) - xPerp * Math.sin(bearingRad);
    const dLon = east / (EARTH_RADIUS_M * Math.cos(latRad));
    const dLat = north / EARTH_RADIUS_M;
    coords.push([lon + (dLon * 180) / Math.PI, lat + (dLat * 180) / Math.PI]);
  }
  return coords;
}

export type WakePreset = "6x4" | "5x3";

const WAKE_PRESETS: Record<WakePreset, { semiMajorFactor: number; semiMinorFactor: number }> = {
  "6x4": { semiMajorFactor: 3, semiMinorFactor: 2 },
  "5x3": { semiMajorFactor: 2.5, semiMinorFactor: 1.5 },
};

export interface TurbineAreas {
  sweptAreas: GeoJSON.FeatureCollection;
  wakeEllipses: GeoJSON.FeatureCollection;
}

/**
 * Generates swept area circles (radius = rotor_diameter / 2) and wake ellipses
 * for each turbine in a layout. Accepts the per-turbine GeoJSON FeatureCollection
 * returned by the geojson download endpoint. Turbines without a rotor_diameter_mm
 * are skipped. Returns null if no turbines have a rotor diameter set.
 */
export function generateTurbineAreas(
  fc: GeoJSON.FeatureCollection,
  wakePreset: WakePreset = "6x4",
  windFromDeg = 225,
): TurbineAreas | null {
  const wakeBearing = ((windFromDeg + 180) % 360) * (Math.PI / 180);
  const { semiMajorFactor, semiMinorFactor } = WAKE_PRESETS[wakePreset];

  const sweptFeatures: GeoJSON.Feature[] = [];
  const wakeFeatures: GeoJSON.Feature[] = [];

  for (const feature of fc.features) {
    const geom = feature.geometry;
    if (!geom || geom.type !== "Point") continue;

    const rotorDiameterMm = feature.properties?.rotor_diameter_mm;
    if (typeof rotorDiameterMm !== "number") continue;

    const [lon, lat] = geom.coordinates;
    const rotorDiameterM = rotorDiameterMm / 1000;
    const sweptRadius = rotorDiameterM / 2;

    sweptFeatures.push({
      type: "Feature",
      geometry: {
        type: "Polygon",
        coordinates: [generateEllipse(lon, lat, sweptRadius, sweptRadius, 0)],
      },
      properties: {},
    });
    wakeFeatures.push({
      type: "Feature",
      geometry: {
        type: "Polygon",
        coordinates: [
          generateEllipse(
            lon,
            lat,
            semiMajorFactor * rotorDiameterM,
            semiMinorFactor * rotorDiameterM,
            wakeBearing,
          ),
        ],
      },
      properties: {},
    });
  }

  if (sweptFeatures.length === 0) return null;

  return {
    sweptAreas: { type: "FeatureCollection", features: sweptFeatures },
    wakeEllipses: { type: "FeatureCollection", features: wakeFeatures },
  };
}
