import "./style.css";
import { a, b } from "../lib/main";

import sog_path from "../../../crates/sample_data/pizza.sog?url";

async function main() {
  const sog_file = await fetch(sog_path).then((res) => res.arrayBuffer());
  const sog = a(new Uint8Array(sog_file));
  using splat = b(sog);

  console.log(splat);
}

main();
