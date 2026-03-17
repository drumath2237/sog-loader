import { GaussianSplattingMesh, type Scene, Vector3 } from "@babylonjs/core";
import {
  decodeRaw,
  type RawSplat,
  type Splat,
  unpackRaw,
} from "@sog-loader/core";

export async function createGsFromSogFile(
  sogFile: ArrayBuffer,
  scene: Scene,
): Promise<GaussianSplattingMesh> {
  const sogData = unpackRaw(new Uint8Array(sogFile));
  using splat = decodeRaw(sogData);
  const binarySplat = _convertSplatToSPlatBinary(splat);
  const sh = _createShTextureBuffers(splat, scene);
  const gsMesh = new GaussianSplattingMesh("splat", undefined, scene, true);
  await gsMesh.updateDataAsync(binarySplat, sh ?? undefined);
  gsMesh.scaling = new Vector3(1, -1, 1);
  return gsMesh;
}

function _convertSplatToSPlatBinary(splat: Splat | RawSplat): ArrayBuffer {
  const splatCount = splat.count;
  const unitByteLength = 4 * 3 + 4 * 3 + 4 + 4; // = 32
  const data = new ArrayBuffer(splatCount * unitByteLength);
  const SH_C0 = 0.28209479177387814; // SH_C0 = Y_0^0 = 1 / (2 * sqrt(pi))

  const position = splat.position;
  const scale = splat.scale;
  const rotation = splat.rotation;
  const sh0 = splat.sh0;

  const f32View = new Float32Array(data);
  const u8View = new Uint8ClampedArray(data);

  for (let i = 0; i < splatCount; i++) {
    const splatOffset32 = i * (unitByteLength / 4); // 32 bytes per splat / 4 bytes per float
    const splatOffset8 = i * unitByteLength; // 32 bytes per splat

    // Position
    f32View[splatOffset32 + 0] = position[i * 3 + 0];
    f32View[splatOffset32 + 1] = position[i * 3 + 1];
    f32View[splatOffset32 + 2] = position[i * 3 + 2];

    // Scale
    f32View[splatOffset32 + 3] = Math.exp(scale[i * 3 + 0]);
    f32View[splatOffset32 + 4] = Math.exp(scale[i * 3 + 1]);
    f32View[splatOffset32 + 5] = Math.exp(scale[i * 3 + 2]);

    // Color
    u8View[splatOffset8 + 24] = (SH_C0 * sh0[i * 4 + 0] + 0.5) * 255;
    u8View[splatOffset8 + 25] = (SH_C0 * sh0[i * 4 + 1] + 0.5) * 255;
    u8View[splatOffset8 + 26] = (SH_C0 * sh0[i * 4 + 2] + 0.5) * 255;
    u8View[splatOffset8 + 27] = (1 / (1 + Math.exp(-sh0[i * 4 + 3]))) * 255;

    // Rotation
    u8View[splatOffset8 + 28] = rotation[i * 4 + 0] * 127.5 + 127.5;
    u8View[splatOffset8 + 29] = rotation[i * 4 + 1] * 127.5 + 127.5;
    u8View[splatOffset8 + 30] = rotation[i * 4 + 2] * 127.5 + 127.5;
    u8View[splatOffset8 + 31] = rotation[i * 4 + 3] * 127.5 + 127.5;
  }

  return data;
}
function _createShTextureBuffers(
  splat:
    | { shN: Float32Array; count: number; sh_degree: number }
    | Splat
    | RawSplat,
  scene: Scene,
): Array<Uint8Array> | null {
  const shN = splat.shN;
  const sh_degree = splat.sh_degree;
  const splatCount = splat.count;

  if (!shN) {
    return null;
  }

  let coeffCount = 0;
  switch (sh_degree) {
    case 0:
      return null;
    case 1:
      coeffCount = 3;
      break;
    case 2:
      coeffCount = 8;
      break;
    case 3:
      coeffCount = 15;
      break;
    default:
      return null;
  }
  const componentsCount = coeffCount * 3;
  const textureCount = Math.ceil(componentsCount / (4 * 4)); // 4 components can be stored per texture, 4 sh per component
  const textureWidth = scene.getEngine().getCaps().maxTextureSize;
  const textureHeight = Math.ceil(splatCount / textureWidth);
  const shTextureBuffers = Array.from(
    { length: textureCount },
    () => new Uint8Array(textureWidth * textureHeight * 4 * 4),
  );

  for (let i = 0; i < splatCount; i++) {
    const componentOffset = 4 * 4 * i;
    for (let j = 0; j < componentsCount; j++) {
      const buffer = shTextureBuffers[Math.floor(j / (4 * 4))];
      const componentIndexInTexture = j % (4 * 4);
      const index = (j % 3) * coeffCount + Math.floor(j / 3);
      const shValue = shN[i * componentsCount + index] * 127.5 + 127.5;
      buffer[componentOffset + componentIndexInTexture] = Math.min(
        Math.max(0, shValue),
        255,
      );
    }
  }

  return shTextureBuffers;
}
