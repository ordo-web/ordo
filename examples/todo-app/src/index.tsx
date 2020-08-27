import * as React from "react";
import * as ReactDOM from "react-dom";
import { Provider } from "ordo-react";
import TodoApp from "./todoApp";
// import { addTodo } from "../bin/ordo-bindings/TodoAction";

const ordoCore = new Worker("worker.js");

import("../../../ordo_adapter/pkg/ordo_adapter").then(async (ordo) => {
  const node = new ordo.Node(ordoCore);
  await node.ready();

  /**
  store.subscribe(() => {
    const state = store.getState();
    console.log(state);
  });
  store.dispatch(addTodo({ id: 10, content: "abc" }));*/

  ReactDOM.render(
    <Provider compiler="TypeScript" framework="React" node={node}>
      <TodoApp />
    </Provider>,
    document.getElementById("root")
  );
});
