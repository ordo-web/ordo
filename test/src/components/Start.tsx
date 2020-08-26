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

  runOrdoTasks() {
    this.props.testFunc(this.props.store);
  }

  render() {
    let render;
    if (!this.state.initialized) {
      render = <button onClick={this.handleClick}>Start</button>;
    } else {
      render = this.props.component;
    }

    return <div>{render}</div>;
  }
}
