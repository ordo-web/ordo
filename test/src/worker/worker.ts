import("../../bin/pkg/ordo_test").then((wasm) => {
  //wasm.logging();
  //wasm.test();
  const myApp = new wasm.SingleStoreExample();
});
