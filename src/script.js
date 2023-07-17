import init, {BoardGenerator} from '../pkg/loteria.js';

async function run() {
  const wasm = await init().catch(console.error);
  let start = Date.now();
  let generator = new BoardGenerator(4, 4);
  let end = Date.now();

  console.log("delta: " + (end - start) + "ms");
}

run();
