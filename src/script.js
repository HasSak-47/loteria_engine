import init, {BoardGenerator} from '../pkg/loteria.js';

async function run() {
  const wasm = await init().catch(console.error);
  let start = Date.now();
  let generator = new BoardGenerator(4, 4);
  let end = Date.now();

  console.log("delta: " + (end - start) + "ms");

  let x = generator.next();
  while(x != undefined){
    let vals = new Uint8Array(16);
    for(let i = 0; i < 16; ++i){
      vals[i] = x.get(i);
    }
    console.log(vals);
    x = generator.next();
  }
}

run();
