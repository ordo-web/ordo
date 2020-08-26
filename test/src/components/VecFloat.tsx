import * as React from "react";

export default class VecFloat extends React.Component<any, any> {
  constructor(props: any) {
    super(props);

    const ordoState = props.store.getState();
    this.state = {
      vec: ordoState.vecState.vec,
      float: ordoState.structState.number.number,
    };

    this.handleStateChange = this.handleStateChange.bind(this);
    props.store.subscribe(this.handleStateChange);
  }

  handleStateChange() {
    const state = this.props.store.getState();
    this.setState({
      vec: state.vecState.vec,
      float: state.structState.number.number,
    });
  }

  render() {
    return (
      <div>
        <h2>Vec</h2>
        <h1>{this.state.vec}</h1>
        <h2>Float</h2>
        <h1>{this.state.float}</h1>
      </div>
    );
  }
}
