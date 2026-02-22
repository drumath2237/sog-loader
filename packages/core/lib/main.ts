import {
  decode as decodeRaw,
  type RawSogDataV2,
  RawSplat,
  unpack as unpackRaw,
} from "./wasm/sog_decoder_wasm";

export { unpackRaw, decodeRaw };

export function decode(data: RawSogDataV2): Splat {
  using rawSplat = decodeRaw(data);
  return rawSplat.clone();
}

export type Splat = {
  antialias: boolean;
  count: number;
  position: Float32Array;
  rotation: Float32Array;
  scale: Float32Array;
  sh0: Float32Array;
  shN?: Float32Array;
  sh_degree: number;
};

declare module "./wasm/sog_decoder_wasm" {
  interface RawSplat {
    clone(): Splat;
  }
}

RawSplat.prototype.clone = function (): Splat {
  const rawSplat = this as RawSplat;
  return {
    antialias: rawSplat.antialias,
    count: rawSplat.count,
    sh_degree: rawSplat.sh_degree,
    position: new Float32Array(rawSplat.position),
    rotation: new Float32Array(rawSplat.rotation),
    scale: new Float32Array(rawSplat.scale),
    sh0: new Float32Array(rawSplat.sh0),
    shN:
      rawSplat.shN === undefined ? undefined : new Float32Array(rawSplat.shN),
  };
};
