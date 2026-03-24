import "./style.css";
import { Engine, Scene, Vector3 } from "@babylonjs/core";
import sog_path from "../../../crates/sample_data/hornedlizard.sog?url";
import { createGsFromSogFile } from "../lib";

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
    // gs1.position = new Vector3(0.22, 0, 0);
    gs1.scaling = new Vector3(-1, 1, 1);
  };

  await Promise.all([task1()]);
}

main();
