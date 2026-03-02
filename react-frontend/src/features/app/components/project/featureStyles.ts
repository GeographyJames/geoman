import { Stroke, Fill, Style, Circle } from "ol/style";

export const primaryStyle = new Style({
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

export const defaultStyle = new Style({
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

export const sweptAreaStyle = new Style({
  stroke: new Stroke({ color: "rgba(37, 99, 235, 0.5)", width: 1 }),
  fill: new Fill({ color: "rgba(37, 99, 235, 0.06)" }),
});

export const wakeEllipseStyle = new Style({
  stroke: new Stroke({
    color: "rgba(217, 119, 6, 0.6)",
    width: 1,
    lineDash: [4, 4],
  }),
  fill: new Fill({ color: "rgba(217, 119, 6, 0.04)" }),
});
