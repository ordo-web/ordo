import * as React from "react";
import * as ReactDOM from "react-dom";

import Selector from "./components/Selector";

/**
// web worker
const worker = new Worker("worker.js");

import("../../ordo_adapter/pkg/ordo_adapter").then(async (ordo) => {
  const store = new ordo.Node(worker);

  await store.ready();

  ReactDOM.render(
    <Start compiler="TypeScript" framework="React" store={store} />,
    document.getElementById("root")
  );
});*/

ReactDOM.render(
  <Selector compiler="TypeScript" framework="React" />,
  document.getElementById("root")
);
