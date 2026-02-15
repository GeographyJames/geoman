import { useApiRequest } from "@/lib/api";
import { useMutation } from "@tanstack/react-query";

export interface CrsInfo {
  srid: number;
  name: string | null;
}

export const usePostEpsg = () => {
  const apiRequest = useApiRequest();

  return useMutation<CrsInfo, Error, string>({
    mutationFn: async (prj) => {
      const data = await apiRequest<CrsInfo>("/api/epsg", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ prj }),
      });
      return data!;
    },
  });
};
