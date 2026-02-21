import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
  build: {
    lib: {
      entry: "./lib/main.ts",
      name: "Counter",
      fileName: "counter",
      formats: ["es", "system"],
    },
  },
});
