import { defineConfig } from 'vite'
import viteReact from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

import { tanstackRouter } from '@tanstack/router-plugin/vite'
import { fileURLToPath, URL } from 'node:url'

import fs from "fs"
import YAML from "yaml"
import dotenv from "dotenv"

const urls_file = fs.readFileSync("../config/urls.yaml", "utf8")
const urls_yaml = YAML.parse(urls_file)

// Load environment variables from parent directory .env file
const envConfig = dotenv.config({ path: "../.env" })
const runEnvironment = envConfig.parsed?.GEOMAN_RUN_ENVIRONMENT || "development"


// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  return {
    plugins: [
      tanstackRouter({
        target: 'react',
        autoCodeSplitting: true,
      }),
      viteReact(),
      tailwindcss(),
    ],
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url)),
      },
    },
    test: {
      globals: true,
      environment: "jsdom",
      setupFiles: "./tests/setup.ts"
    },
    define: {
      __URLS__: JSON.stringify(urls_yaml),
      __RUN_ENVIRONMENT__: JSON.stringify(runEnvironment),
    },
    server: {
      proxy: mode === 'development' ? {
        '/api': { target: "http://localhost:8000", changeOrigin: true, secure: false },
        '/ogcapi': { target: "http://localhost:8000", changeOrigin: true, secure: false },

      } : undefined
    },

  }
})
