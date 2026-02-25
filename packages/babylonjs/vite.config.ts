import { resolve } from "path";
import dts from "unplugin-dts/vite";
import { defineConfig } from "vite";

export default defineConfig({
  // const packageName = await readPackageJSON().then((json) => json.name);
  // if (!packageName) {
  //   throw new Error("name is not defined in package.json");
  // }

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
      name: "package",
      fileName: "index",
      formats: ["es", "umd"],
    },
  },
});
