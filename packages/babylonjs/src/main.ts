import "./style.css";
import { Engine, Scene } from "@babylonjs/core";
import sog_path from "../../../crates/sample_data/pizza.sog?url";
import { createGsFromSogFile } from "../lib";

const main = () => {
  const renderCanvas =
    document.querySelector<HTMLCanvasElement>("#renderCanvas");
  if (!renderCanvas) {
    return;
  }

  const engine = new Engine(renderCanvas);
  const scene = new Scene(engine);

  scene.createDefaultCameraOrLight(true, true, true);
  // scene.createDefaultEnvironment();

  fetch(sog_path)
    .then((res) => res.arrayBuffer())
    .then((sogfile) => createGsFromSogFile(sogfile, scene));

  window.addEventListener("resize", () => engine.resize());
  engine.runRenderLoop(() => scene.render());
};

main();
