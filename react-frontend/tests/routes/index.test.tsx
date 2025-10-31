import {
  createMemoryHistory,
  createRouter,
  RouterProvider,
} from "@tanstack/react-router";
import { QueryClient } from "@tanstack/react-query";

import { routeTree } from "@/routeTree.gen";
import { describe, expect, test } from "vitest";
import { render, screen } from "@testing-library/react";
import { ClerkProvider } from "@clerk/clerk-react";

// Import Clerk Publishable Key
const CLERK_TEST_KEY = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;

if (!CLERK_TEST_KEY) {
  throw new Error("Add your Clerk Publishable Key to the .env file");
}

describe("Routing", () => {
  test('shows the Clerk login page when visiting "/" as an unauthenticated user', async () => {
    const queryClient = new QueryClient({
      defaultOptions: {
        queries: {
          retry: false, // Don't retry failed queries in tests
        },
      },
    });

    const history = createMemoryHistory({ initialEntries: ["/"] });
    const router = createRouter({
      routeTree,
      history,
      context: {
        queryClient,
      },
    });

    render(
      <ClerkProvider publishableKey={CLERK_TEST_KEY}>
        <RouterProvider router={router} />
      </ClerkProvider>
    );
    expect(await screen.findByText("Sign in")).toBeInTheDocument();
  });
});
