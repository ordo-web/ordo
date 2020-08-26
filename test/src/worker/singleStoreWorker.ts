import("../../bin/pkg/ordo_test").then((wasm) => {
  const myApp = new wasm.SingleStoreAsyncExample();
  myApp.testDispatch();
});
