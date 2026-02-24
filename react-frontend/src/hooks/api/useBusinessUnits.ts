import { useQuery } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";
import type { BusinessUnitOutputDto } from "@/domain/business_unit/outputDto";

export function useBusinessUnits() {
  const apiRequest = useApiRequest();
  const url = __URLS__.api.base + __URLS__.api.business_units;
  return useQuery({
    queryKey: ["business_units"],
    queryFn: () => apiRequest<BusinessUnitOutputDto[]>(url).then(d => d ?? []),
  });
}
