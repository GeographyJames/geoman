import { useApiRequest } from "@/lib/api";
import { useMutation } from "@tanstack/react-query";
import type { CrsInfo } from "./usePostEpsg";

export const usePostEpsgFromShz = () => {
  const apiRequest = useApiRequest();

  return useMutation<CrsInfo, Error, File>({
    mutationFn: async (shzFile) => {
      const form = new FormData();
      form.append("shz", shzFile);
      const data = await apiRequest<CrsInfo>("/api/epsg/shz", {
        method: "POST",
        body: form,
      });
      return data!;
    },
  });
};
