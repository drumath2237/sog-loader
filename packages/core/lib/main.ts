import { decode, SogDataV2, unpack } from "./wasm/sog_decoder_wasm";

export function a(file: Uint8Array) {
  return unpack(file);
}

export function b(sog: SogDataV2) {
  return decode(sog);
}
