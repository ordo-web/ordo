import * as React from "react";
import Counter from "./Counter";
import { sleep } from "../tools/utils";
import Start from "./Start";
import Text from "./Text";
// @ts-ignore
import * as CounterAction from "../../bin/ordo-bindings/CounterAction.js";
// @ts-ignore
import * as TextAction from "../../bin/ordo-bindings/TextAction.js";
// @ts-ignore
import * as VecAction from "../../bin/ordo-bindings/VecAction.js";
// @ts-ignore
import * as FloatAction from "../../bin/ordo-bindings/FloatAction.js";
import VecFloat from "./VecFloat";

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

      case "combinedStoreSync":
        // web worker
        worker = new Worker("combinedStoreSync.js");

        // @ts-ignore
        import("../../../ordo_adapter/pkg/ordo_adapter").then(async (ordo) => {
          const store = new ordo.Node(worker);
          await store.ready();
          const testFunc = async (store: any) => {
            await sleep(500);
            store.dispatch(VecAction.push(10));
            await sleep(500);
            store.dispatch(VecAction.pop());
            await sleep(500);
            store.dispatch(FloatAction.multiply(10.0));
            await sleep(500);
            store.dispatch(FloatAction.divide(10.0));
          };
          const component = <VecFloat store={store} />;

          this.setState({
            testName: testName,
            testFunc: testFunc,
            store: store,
            component: component,
          });
        });
        break;
      case "combinedStoreAsync":
        // web worker
        worker = new Worker("combinedStoreAsync.js");

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
      case "combinedStoreWorker":
        // web worker
        worker = new Worker("combinedStoreWorker.js");

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
          <br />
          <br />
          <button onClick={() => this.selectTest("combinedStoreSync")}>
            combinedStoreSync
          </button>
          <br />
          <button onClick={() => this.selectTest("combinedStoreAsync")}>
            combinedStoreAsync
          </button>
          <br />
          <button onClick={() => this.selectTest("combinedStoreWorker")}>
            combinedStoreWorker
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
