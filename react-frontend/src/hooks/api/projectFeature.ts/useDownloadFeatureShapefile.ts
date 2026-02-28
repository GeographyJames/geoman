import { useAuth } from "@clerk/clerk-react";
import { useFlash } from "@/features/app/contexts/FlashMessageContext";
import { useState } from "react";

export const useDownloadFeatureShapefile = () => {
  const { getToken, isSignedIn } = useAuth();
  const { addFlash } = useFlash();
  const [isLoading, setIsLoading] = useState(false);

  const download = async (featureId: number, projectSlug: string, collectionSlug: string, format: "shapefile" | "csv") => {
    setIsLoading(true);
    try {
      let token: string | null = null;
      if (isSignedIn) {
        token = await getToken();
      }

      const response = await fetch(`${__URLS__.api.base}${__URLS__.api.project_features}/${projectSlug}/${collectionSlug}/${featureId}?format=${format}`, {
        headers: {
          ...(token && { Authorization: `Bearer ${token}` }),
        },
      });

      if (!response.ok) {
        const contentType = response.headers.get("content-type");
        let message = `Download failed (${response.status})`;
        if (contentType?.includes("application/json")) {
          try {
            const errorJson = await response.json();
            if (typeof errorJson?.message === "string") {
              message = errorJson.message;
            }
          } catch {
            // ignore
          }
        }
        addFlash(message, "error");
        return;
      }

      const blob = await response.blob();
      const disposition = response.headers.get("Content-Disposition");
      const filenameMatch = disposition?.match(/filename="(.+)"/);
      const ext = format === "csv" ? "csv" : "shz";
      const filename = filenameMatch?.[1] ?? `feature-${featureId}.${ext}`;

      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);
    } catch (error) {
      addFlash(
        `Download failed: ${error instanceof Error ? error.message : "Unknown error"}`,
        "error",
      );
    } finally {
      setIsLoading(false);
    }
  };

  return { download, isLoading };
};
