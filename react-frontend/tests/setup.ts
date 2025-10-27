import '@testing-library/jest-dom/vitest'
import { vi } from 'vitest'

// Mock TanStack Devtools to prevent mounting issues in tests
vi.mock('@tanstack/react-devtools', () => ({
  TanStackDevtools: () => null,
}))

vi.mock('@tanstack/react-router-devtools', () => ({
  TanStackRouterDevtoolsPanel: () => null,
}))