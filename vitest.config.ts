import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  resolve: {
    alias: {
      "@": resolve(__dirname, "./src"),
    },
  },
  test: {
    environment: 'happy-dom',
    include: ['tests/unit/**/*.{test,spec}.ts', 'tests/dom/**/*.{test,spec}.ts'],
    exclude: ['tests/e2e/**'],
  },
});