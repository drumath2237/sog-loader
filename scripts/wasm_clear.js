const fs = require("fs");

if (fs.existsSync("lib/wasm")) {
  fs.rm("lib/wasm", { recursive: true }, () => {});
}
