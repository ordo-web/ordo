import * as React from "react";
import Counter from "./Counter";
// @ts-ignore
import * as CounterAction from "../../bin/ordo-bindings/CounterAction.js";
import { sleep } from "../tools/utils";
import Start from "./Start";

export default class Selector extends React.Component<any, any> {
  constructor(props: any) {
    super(props);

    this.state = {
      testName: undefined,
      testFunc: undefined,
      store: undefined,
      component: undefined,
    };

    this.selectTest = this.selectTest.bind(this);
  }

  selectTest(testName: String) {
    switch (testName) {
      case "singleStoreSync":
        // web worker
        const worker = new Worker("worker.js");

        // @ts-ignore
        import("../../../ordo_adapter/pkg/ordo_adapter").then(async (ordo) => {
          const store = new ordo.Node(worker);
          await store.ready();
          const testFunc = async (store: any) => {
            await sleep(500);
            store.dispatch(CounterAction.increment());
            await sleep(500);
            store.dispatch(CounterAction.decrement());
          };
          const component = <Counter store={store} />;

          this.setState({
            testName: testName,
            testFunc: testFunc,
            store: store,
            component: component,
          });
        });
    }
  }

  render() {
    if (this.state.testName === undefined) {
      return (
        <div>
          <h2>Tests</h2>
          <button onClick={() => this.selectTest("singleStoreSync")}>
            singleStoreSync
          </button>
        </div>
      );
    } else {
      return (
        <div>
          <Start
            store={this.state.store}
            testFunc={this.state.testFunc}
            component={this.state.component}
          />
        </div>
      );
    }
  }
}
