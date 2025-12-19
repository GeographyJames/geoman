import LandingPage from "@/components/LandingPage";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/about")({
  component: App,
});

export default function App() {
  return <LandingPage />;
}
