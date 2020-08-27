import * as React from "react";
import { Node } from "ordo_adapter";
import * as PropTypes from "prop-types";

type ProviderProps = {
  node: Node;
};

export class Provider extends React.Component<ProviderProps, {}> {
  constructor(props: ProviderProps) {
    super(props);
  }

  static childContextTypes = {
    node: PropTypes.object.isRequired,
  };

  getChildContext() {
    const { node } = this.props;
    return { node };
  }

  render() {
    return this.props.children;
  }
}
