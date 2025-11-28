import { createFileRoute } from "@tanstack/react-router";
import { ApiReferenceReact } from "@scalar/api-reference-react";
import "@scalar/api-reference-react/style.css";

export const Route = createFileRoute("/docs")({
  component: ApiDocs,
});

function ApiDocs() {
  return (
    <ApiReferenceReact
      configuration={{
        url: "/ogcapi/openapi.json",
      }}
    />
  );
}
