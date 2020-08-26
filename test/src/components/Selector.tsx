import * as React from "react";
import Counter from "./Counter";
// @ts-ignore
import * as CounterAction from "../../bin/ordo-bindings/CounterAction.js";
// @ts-ignore
import * as TextAction from "../../bin/ordo-bindings/TextAction.js";
import { sleep } from "../tools/utils";
import Start from "./Start";
import Text from "./Text";

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
    let worker: Worker;
    switch (testName) {
      case "singleStoreSync":
        // web worker
        worker = new Worker("singleStoreSync.js");

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
        break;
      case "singleStoreAsync":
        // web worker
        worker = new Worker("singleStoreAsync.js");

        // @ts-ignore
        import("../../../ordo_adapter/pkg/ordo_adapter").then(async (ordo) => {
          const store = new ordo.Node(worker);
          await store.ready();
          const testFunc = async (store: any) => {
            await sleep(500);
            store.dispatch(TextAction.replace("Hello World!"));
            await sleep(500);
            store.dispatch(TextAction.reset());
          };
          const component = <Text store={store} />;

          this.setState({
            testName: testName,
            testFunc: testFunc,
            store: store,
            component: component,
          });
        });
        break;
      case "singleStoreWorker":
        // web worker
        worker = new Worker("singleStoreWorker.js");

        // @ts-ignore
        import("../../../ordo_adapter/pkg/ordo_adapter").then(async (ordo) => {
          const store = new ordo.Node(worker);
          await store.ready();
          const testFunc = async () => {};
          const component = <Text store={store} />;

          this.setState({
            testName: testName,
            testFunc: testFunc,
            store: store,
            component: component,
          });
        });
        break;
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
          <br />
          <button onClick={() => this.selectTest("singleStoreAsync")}>
            singleStoreAsync
          </button>
          <br />
          <button onClick={() => this.selectTest("singleStoreWorker")}>
            singleStoreWorker
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
