import "./style.css";
import {
  Engine,
  type GaussianSplattingMesh,
  ImportMeshAsync,
  Scene,
  Vector3,
} from "@babylonjs/core";
import sog_path from "../../../crates/sample_data/pizza.sog?url";
import { createGsFromSogFile } from "../lib";
import "@babylonjs/loaders/SPLAT";

async function main() {
  const renderCanvas =
    document.querySelector<HTMLCanvasElement>("#renderCanvas");
  if (!renderCanvas) {
    return;
  }

  const engine = new Engine(renderCanvas);
  const scene = new Scene(engine);

  scene.createDefaultCameraOrLight(true, true, true);

  window.addEventListener("resize", () => engine.resize());
  engine.runRenderLoop(() => scene.render());

  console.log("====================");

  const task1 = async () => {
    console.time("sog-loader");
    const gs1 = await fetch(sog_path)
      .then((res) => res.arrayBuffer())
      .then((sogfile) => createGsFromSogFile(sogfile, scene));
    console.timeEnd("sog-loader");
    gs1.position = new Vector3(0.22, 0, 0);
  };

  const task2 = async () => {
    console.time("babylonjs");
    const gs2 = await ImportMeshAsync(sog_path, scene).then(
      (res) => res.meshes[0] as GaussianSplattingMesh,
    );
    console.timeEnd("babylonjs");
    gs2.position = new Vector3(-0.22, 0, 0);
  };

  await Promise.all([task1(), task2()]);
}

main();
