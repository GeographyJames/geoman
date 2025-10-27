import {
  createMemoryHistory,
  createRouter,
  RouterProvider,
} from "@tanstack/react-router";
import { QueryClient } from "@tanstack/react-query";

import { routeTree } from "@/routeTree.gen";
import { describe, expect, test } from "vitest";
import { render, screen } from "@testing-library/react";

describe("Routing", () => {
  test('shows the welcome page when visiting "/"', async () => {
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

    render(<RouterProvider router={router} />);
    expect(await screen.findByText("Learn React")).toBeInTheDocument();
  });
});
