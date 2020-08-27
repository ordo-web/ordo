import * as React from "react";
import * as ReactDOM from "react-dom";
import { Provider } from "react-redux";
import TodoApp from "./todoApp";

import { addTodo } from "../bin/ordo-bindings/TodoAction";

const ordoCore = new Worker("worker.js");

class StoreWrapper {
  node: any;
  constructor(node: any) {
    this.node = node;
  }

  getState() {
    return this.node.getState();
  }

  dispatch(action: any) {
    return this.node.dispatch(action);
  }

  subscribe(func: any) {
    this.node.subscribe(func);
  }

  unsubscribe(func: any) {
    this.node.unsubscribe(func);
  }
}

import("../../../ordo_adapter/pkg/ordo_adapter").then(async (ordo) => {
  const store = new ordo.Node(ordoCore);
  await store.ready();

  let wrapper = new StoreWrapper(store);

  /**
  store.subscribe(() => {
    const state = store.getState();
    console.log(state);
  });
  store.dispatch(addTodo({ id: 10, content: "abc" }));*/

  ReactDOM.render(
    <Provider compiler="TypeScript" framework="React" store={wrapper}>
      <TodoApp />
    </Provider>,
    document.getElementById("root")
  );
});
