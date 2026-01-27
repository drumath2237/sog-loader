import wasm from "vite-plugin-wasm";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [wasm()],
  build: {
    lib: {
      entry: "./lib/main.ts",
      name: "Counter",
      fileName: "counter",
    },
  },
});
