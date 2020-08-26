import * as React from "react";
import * as ReactDOM from "react-dom";
import { Provider } from "react-redux";
import TodoApp from "./todoApp";

const ordoCore = new Worker("worker.js");

import("../../../ordo_adapter/pkg/ordo_adapter").then(async (ordo) => {
  const store = new ordo.Node(ordoCore);
  await store.ready();

  ReactDOM.render(
    <Provider compiler="TypeScript" framework="React" store={store}>
      <TodoApp />
    </Provider>,
    document.getElementById("root")
  );
});
