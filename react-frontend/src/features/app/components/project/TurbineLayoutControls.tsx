import type { WakePreset } from "@/lib/turbineAreas";

export function TurbineLayoutControls({
  name,
  wakePreset,
  setWakePreset,
  windFromDeg,
  setWindFromDeg,
}: {
  name: string;
  wakePreset: WakePreset;
  setWakePreset: (preset: WakePreset) => void;
  windFromDeg: number;
  setWindFromDeg: (deg: number) => void;
}) {
  return (
    <div className="flex flex-wrap items-center gap-x-4 gap-y-1 pb-1 text-xs">
      <div className="flex items-center gap-2">
        <span className="text-base-content/70">
          Ellipse size (rotor diameters):
        </span>
        <div className="join">
          <input
            className="join-item btn btn-xs"
            type="radio"
            name={name}
            aria-label="6×4"
            checked={wakePreset === "6x4"}
            onChange={() => setWakePreset("6x4")}
          />
          <input
            className="join-item btn btn-xs"
            type="radio"
            name={name}
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
  );
}
