import "./style.css";

import sog_path from "../../../crates/sample_data/pizza.sog?url";
import { decodeRaw, getDecoderInfo, unpackRaw } from "../lib";

async function main() {
  using info = getDecoderInfo();
  console.log(`version=${info.version}, git HASH=${info.git_hash}`);

  const sog_file = await fetch(sog_path).then((res) => res.arrayBuffer());

  console.log("start");

  const sogRaw = unpackRaw(new Uint8Array(sog_file));
  using splatRaw = decodeRaw(sogRaw);
  console.log(splatRaw);

  console.log("done");

  const splat = splatRaw.clone();
  console.log(splat);
}

main();
