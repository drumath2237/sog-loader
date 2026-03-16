import { resolve } from "node:path";
import dts from "unplugin-dts/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    dts({
      tsconfigPath: "./tsconfig.json",
    }),
  ],

  build: {
    rollupOptions: {
      external: ["@babylonjs/core"],
      output: {
        globals: {
          "@babylonjs/core": "BABYLON",
        },
      },
    },
    sourcemap: true,
    lib: {
      entry: resolve(__dirname, "./lib/index.ts"),
      name: "@sog-loader/babylonjs",
      fileName: "index",
      formats: ["es"],
    },
  },
});
