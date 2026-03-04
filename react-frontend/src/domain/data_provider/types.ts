export type DataProviderServiceType =
  | "ImageWMS"
  | "TileWMS"
  | "WMTS"
  | "WFS"
  | "ArcGISRest"
  | "MVT"
  | "OGCAPIFeatures"
  | "XYZ";

export type LayerCategory = "overlay" | "basemap";

export interface DataProvider {
  id: number;
  name: string;
  description: string | null;
  country_code: string | null;
  subdivision: string | null;
}

export interface DataProviderService {
  id: number;
  provider_id: number;
  name: string;
  service_type: DataProviderServiceType;
  base_url: string;
}

export interface DataProviderLayer {
  id: number;
  service_id: number;
  name: string;
  abbreviation: string | null;
  source: unknown;
  category: LayerCategory;
  description: string | null;
  enabled: boolean;
  style_config: unknown;
  display_options: unknown;
  country_code: string | null;
  subdivision: string | null;
  sort_order: number;
}
