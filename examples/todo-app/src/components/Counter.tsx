import * as React from "react";

export default class Counter extends React.Component<any, any> {
  constructor(props: any) {
    super(props);

    const ordoState = props.store.getState();
    this.state = {
      counter: ordoState.counter,
    };

    this.handleStateChange = this.handleStateChange.bind(this);
    props.store.subscribe(this.handleStateChange);
  }

  handleStateChange() {
    const state = this.props.store.getState();
    this.setState({
      counter: state.counter,
    });
  }

  render() {
    return (
      <div>
        <h2>Counter</h2>
        <h1>{this.state.counter}</h1>
      </div>
    );
  }
}
