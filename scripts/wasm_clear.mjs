import { existsSync, rmSync } from "fs";

if (existsSync("lib/wasm")) {
  rmSync("lib/wasm", { recursive: true, force: true });
}
