import {
  GaussianSplattingMesh,
  MeshBuilder,
  type Scene,
  Vector3,
} from "@babylonjs/core";
import { decode, type RawSplat, type Splat, unpackRaw } from "@sog-loader/core";

export const createSphere = (size: number, position: Vector3) => {
  const sphere = MeshBuilder.CreateSphere("sphere", { diameter: size });
  sphere.position = position;
  return sphere;
};

function _convertSplatToSPlatBinary(splat: Splat | RawSplat): ArrayBuffer {
  const splatCount = splat.count;
  const unitByteLength = 4 * 3 + 4 * 3 + 4 + 4; // = 32
  const data = new ArrayBuffer(splatCount * unitByteLength);
  const SH_C0 = 0.28209479177387814; // SH_C0 = Y_0^0 = 1 / (2 * sqrt(pi))

  for (let i = 0; i < splatCount; i++) {
    const posBuffer = new Float32Array(data, i * unitByteLength, 3);
    posBuffer[0] = splat.position[i * 3 + 0];
    posBuffer[1] = splat.position[i * 3 + 1];
    posBuffer[2] = splat.position[i * 3 + 2];

    const scaleBuffer = new Float32Array(data, i * unitByteLength + 12, 3);
    scaleBuffer[0] = Math.exp(splat.scale[i * 3 + 0]);
    scaleBuffer[1] = Math.exp(splat.scale[i * 3 + 1]);
    scaleBuffer[2] = Math.exp(splat.scale[i * 3 + 2]);

    const colorBuffer = new Uint8ClampedArray(data, i * unitByteLength + 24, 4);
    colorBuffer[0] = (SH_C0 * splat.sh0[i * 4 + 0] + 0.5) * 255;
    colorBuffer[1] = (SH_C0 * splat.sh0[i * 4 + 1] + 0.5) * 255;
    colorBuffer[2] = (SH_C0 * splat.sh0[i * 4 + 2] + 0.5) * 255;
    colorBuffer[3] = (1 / (1 + Math.exp(-splat.sh0[i * 4 + 3]))) * 255;

    const rotBuffer = new Uint8ClampedArray(data, i * unitByteLength + 28, 4);
    rotBuffer[0] = splat.rotation[i * 4 + 0] * 127.5 + 127.5;
    rotBuffer[1] = splat.rotation[i * 4 + 1] * 127.5 + 127.5;
    rotBuffer[2] = splat.rotation[i * 4 + 2] * 127.5 + 127.5;
    rotBuffer[3] = splat.rotation[i * 4 + 3] * 127.5 + 127.5;
  }

  return data;
}

export async function createGsFromSogFile(
  sogFile: ArrayBuffer,
  scene: Scene,
): Promise<GaussianSplattingMesh> {
  using sogData = unpackRaw(new Uint8Array(sogFile));
  const splat = decode(sogData);
  const binarySplat = _convertSplatToSPlatBinary(splat);
  const gsMesh = new GaussianSplattingMesh("splat", undefined, scene, true);
  await gsMesh.updateDataAsync(binarySplat);
  gsMesh.scaling = new Vector3(1, -1, 1);
  return gsMesh;
}

function _createShTextureBuffers(shN: Float32Array): Array<Uint8Array> {
  throw new Error("not yet implemented");
}
