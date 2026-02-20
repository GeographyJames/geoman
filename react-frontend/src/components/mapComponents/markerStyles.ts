import { Icon, Style } from "ol/style";

export const markerSvg = (fill: string) =>
  `data:image/svg+xml,${encodeURIComponent(
    `<svg xmlns="http://www.w3.org/2000/svg" width="25" height="41" viewBox="0 0 25 41">` +
      `<path d="M12.5 0C5.6 0 0 5.6 0 12.5c0 2.4.7 4.6 1.9 6.5L12.5 41l10.6-22C24.3 17.1 25 14.9 25 12.5 25 5.6 19.4 0 12.5 0z" fill="${fill}"/>` +
      `<circle cx="12.5" cy="12.5" r="5" fill="#fff" opacity="0.9"/>` +
      `</svg>`,
  )}`;




export const mapMarker = (fill: string, scale = 1) => [
      new Style({
        image: new Icon({
          anchor: [0.3, 1],
          scale,
          src: "/images/marker-shadow.png",
        }),
      }),
      new Style({
        image: new Icon({
          anchor: [0.5, 1],
          scale,
          src: markerSvg(fill),
        }),
      }),
    ];

export const createMarkerStyles = ({animationSteps, hoverScale, colour}:{animationSteps: number, hoverScale: number, colour: string}) => {
      const scaleSteps = Array.from(
      { length: animationSteps + 1 },
      (_, i) => 1 + (hoverScale - 1) * (i / animationSteps),
    );
    return scaleSteps.map((s) => mapMarker(colour, s))
  
}