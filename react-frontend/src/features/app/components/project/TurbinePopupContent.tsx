function formatMm(mm: number | null): string {
  if (mm == null) return "—";
  return `${(mm / 1000).toLocaleString()}m`;
}

export function TurbinePopupContent({
  turbineNumber,
  hubHeightMm,
  rotorDiameterMm,
}: {
  turbineNumber: number;
  hubHeightMm: number | null;
  rotorDiameterMm: number | null;
}) {
  return (
    <div className="space-y-0.5">
      <p className="font-semibold">Turbine {turbineNumber}</p>
      <p>Hub height: {formatMm(hubHeightMm)}</p>
      <p>Rotor diameter: {formatMm(rotorDiameterMm)}</p>
    </div>
  );
}
