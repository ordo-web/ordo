import * as React from "react";
import * as ReactDOM from "react-dom";
// @ts-ignore
import * as CounterAction from "../bin/ordo-bindings/CounterAction.js";
import { sleep } from "./tools/utils";

import Counter from "./components/Counter";

// web worker
const worker = new Worker("worker.js");

import("../../ordo_adapter/pkg/ordo_adapter").then(async (ordo) => {
  const store = new ordo.Node(worker);

  await store.ready();

  ReactDOM.render(
    <Counter compiler="TypeScript" framework="React" store={store} />,
    document.getElementById("root")
  );

  await sleep(3000);
  store.dispatch(CounterAction.increment());

  await sleep(3000);
  store.dispatch(CounterAction.decrement());
});
