declare module "shpjs" {
  import type { FeatureCollection, GeoJsonProperties, Geometry } from "geojson";

  export function parseShp(shp: ArrayBuffer | DataView, prj?: string | false): Geometry[];
  export function parseDbf(dbf: ArrayBuffer | DataView, cpg?: string): GeoJsonProperties[];
  export function combine(arr: [Geometry[], GeoJsonProperties[]]): FeatureCollection;
  export function parseZip(buffer: ArrayBuffer | Uint8Array, whiteList?: string[]): Promise<FeatureCollection | FeatureCollection[]>;

  function getShapefile(base: string | ArrayBuffer | { shp: ArrayBuffer; dbf?: ArrayBuffer; cpg?: ArrayBuffer; prj?: string }): Promise<FeatureCollection | FeatureCollection[]>;
  export default getShapefile;
}
