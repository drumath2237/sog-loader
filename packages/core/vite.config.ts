import { resolve } from "node:path";
import dts from "unplugin-dts/vite";
import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm(), topLevelAwait(), dts({ tsconfigPath: "./tsconfig.json" })],
  build: {
    sourcemap: true,
    lib: {
      entry: resolve(__dirname, "lib/index.ts"),
      name: "@sog-loader/core",
      fileName: "index",
      formats: ["es"],
    },
  },
});
