/**import("../../bin/pkg/ordo_test")
  .then((wasm) => {
    //const myApp = new wasm.SingleStoreExample();
  })
  .catch((ex) => {
    console.log(ex);
  });*/

const fs = require("fs");
const wasmBin = fs.readFileSync("./bin/pkg/ordo_test_bg.wasm");
const wasmModule = new WebAssembly.Module(wasmBin);
const ordo = new WebAssembly.Instance(wasmModule, []);

const myApp = new ordo.exports.SingleStoreExample();
