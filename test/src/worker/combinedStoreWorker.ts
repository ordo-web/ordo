import("../../bin/pkg/ordo_test").then((wasm) => {
  const myApp = new wasm.CombinedStoreAsyncExample();
  myApp.testDispatch();
});
