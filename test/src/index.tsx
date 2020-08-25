import * as React from "react";
import * as ReactDOM from "react-dom";
// @ts-ignore
import * as CounterAction from "../bin/ordo-bindings/CounterAction.js";

import { Hello } from "./components/Hello";

// web worker
const worker = new Worker("worker.js");

/**worker.addEventListener("message", (e) => {
  console.log("Received from worker: " + e.data);
});*/

import("../../ordo_adapter/pkg/ordo_adapter").then(async (ordo) => {
  const store = new ordo.Node(worker);

  /**
  const func = () => {
    const state = store.getState();
    console.log("This func will be unsubscribed ", state);
  };
  store.subscribe(func);
  store.subscribe(() => {
    const state = store.getState();
    console.log("State-Change! ", state);
  });*/

  await store.ready();

  ReactDOM.render(
    <Hello compiler="TypeScript" framework="React" />,
    document.getElementById("root")
  );

  const state = store.getState();
  console.log("Ordo-Store: ", state);

  store.dispatch(CounterAction.increment());
  //store.unsubscribe(func);
  store.dispatch(CounterAction.decrement());
});
