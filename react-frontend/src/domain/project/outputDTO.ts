import {type ProjectProperties } from "./properties";
import type { Feature, Point } from "geojson";

export interface ProjectOutputDTO extends Feature<Point | null, ProjectProperties> {
    id: number;
}