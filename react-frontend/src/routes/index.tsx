import { SignedIn, SignedOut, SignIn, UserButton } from "@clerk/clerk-react";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: App,
});

export default function App() {
  return (
    <main className="flex justify-center items-center h-full">
      <SignedOut>
        <SignIn />
      </SignedOut>

      <SignedIn>
        <UserButton />
      </SignedIn>
    </main>
  );
}
