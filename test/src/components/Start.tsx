import * as React from "react";
import Counter from "./Counter";
// @ts-ignore
import * as CounterAction from "../../bin/ordo-bindings/CounterAction.js";
import { sleep } from "../tools/utils";

export default class Start extends React.Component<any, any> {
  constructor(props: any) {
    super(props);

    this.state = {
      initialized: false,
    };

    this.handleClick = this.handleClick.bind(this);
    this.runOrdoTasks = this.runOrdoTasks.bind(this);
  }

  handleClick() {
    if (!this.state.initialized) {
      this.setState({
        initialized: true,
      });
      this.runOrdoTasks();
    }
  }

  async runOrdoTasks() {
    await sleep(800);
    this.props.store.dispatch(CounterAction.increment());

    await sleep(800);
    this.props.store.dispatch(CounterAction.decrement());
  }

  render() {
    let render;
    if (!this.state.initialized) {
      render = <button onClick={this.handleClick}>Start</button>;
    } else {
      render = <Counter store={this.props.store} />;
    }

    return <div>{render}</div>;
  }
}
