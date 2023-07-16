import init, {BoardGenerator} from '../pkg/loteria.js';

async function run() {
  const wasm = await init().catch(console.error);
  let start = Date.now();
  let generator = await new BoardGenerator(1, 0);
  let end = Date.now();

  console.log("delta: " + (end - start) + "ms");

  generator.log_tape();
}

run();
