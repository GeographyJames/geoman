import { CACHE_KEY_PROJECTS } from "@/cache_keys";
import type ProjectInputDTO from "@/domain/project/inputDTO";


import { useAuth } from "@clerk/clerk-react";
import { useMutation, useQueryClient } from "@tanstack/react-query"

export const usePostProject = () => {
  const { getToken } = useAuth();
  const queryClient = useQueryClient();

  const postProject = async (dto: ProjectInputDTO): Promise<number> => {
    const token = await getToken();
    const response = await fetch("/api/projects", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(dto),
    });

  if (!response.ok) {

    if (response.status >= 500) {
      try {
        const text = await response.text();
        console.error("Server error creating project:", {
          status: response.status,
          body: text,
        });
      } catch {
        console.error("Server error creating project:", response.status);
      }

      throw new Error("Unexpected error");
    }


    let message = "Failed to create project";
    const contentType = response.headers.get("content-type");

    if (contentType?.includes("application/json")) {
      try {
        const errorJson = await response.json();
        if (typeof errorJson?.message === "string") {
          message = errorJson.message;
        }
      } catch {
        // ignore
      }
    } else {
      try {
        const text = await response.text();
        if (text) message = text;
      } catch {
        // ignore
      }
    }

    throw new Error(message);
  }

    const data = await response.json();
    return data.id;
  };

  return useMutation<number, Error, ProjectInputDTO>({
    mutationFn: postProject,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_PROJECTS });
    },
  });
};