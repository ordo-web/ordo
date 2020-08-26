import * as React from "react";

export default class Text extends React.Component<any, any> {
  constructor(props: any) {
    super(props);

    const ordoState = props.store.getState();
    this.state = {
      text: ordoState.text,
    };

    this.handleStateChange = this.handleStateChange.bind(this);
    props.store.subscribe(this.handleStateChange);
  }

  handleStateChange() {
    const state = this.props.store.getState();
    this.setState({
      text: state.text,
    });
  }

  render() {
    return (
      <div>
        <h2>Text</h2>
        <h1>{this.state.text}</h1>
      </div>
    );
  }
}
