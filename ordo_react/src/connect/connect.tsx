import * as React from "react";
import { Node } from "ordo_adapter";
import * as PropTypes from "prop-types";
import { isEqual } from "lodash";

export function connect(mapStateToProps?: any, mapDispatchToProps?: any) {
  return function (WrappedComponent: React.ComponentType) {
    return class HOC extends React.Component<any, any> {
      constructor(props, context) {
        super(props, context);
        const node: Node = this.context.node;
        const currentState = node.getState();
        const mstp = mapStateToPropsCheck(
          mapStateToProps,
          currentState,
          this.props
        );
        const mdtp = mapDispatchToPropsCheck(
          mapDispatchToProps,
          node,
          this.props
        );
        this.state = {
          mstp: mstp,
          mdtp: mdtp,
        };
        console.log("state", this.state);

        this.handleChange = this.handleChange.bind(this);
      }

      static contextTypes = {
        node: PropTypes.object,
      };

      componentDidMount() {
        this.context.node.subscribe(this.handleChange);
      }

      componentWillUnmount() {
        this.context.node.unsubscribe(this.handleChange);
      }

      handleChange() {
        const node: Node = this.context.node;
        const currentState = node.getState();
        const newMstp = mapStateToPropsCheck(
          mapStateToProps,
          currentState,
          this.props
        );
        const componentState = this.state.mstp;
        let change = false;
        for (let key in componentState) {
          if (
            componentState.hasOwnProperty(key) &&
            newMstp.hasOwnProperty(key)
          ) {
            if (!isEqual(componentState[key], newMstp[key])) {
              componentState[key] = newMstp[key];
              change = true;
            }
          }
        }
        if (change) {
          console.log("Render!");
          this.setState({
            mstp: componentState,
          });
        }
      }

      render() {
        return (
          <WrappedComponent
            {...this.props}
            {...this.state.mstp}
            {...this.state.mdtp}
          />
        );
      }
    };
  };
}

function mapStateToPropsCheck(mstp: any, state: Object, props: any): Object {
  if (typeof mstp === "function") {
    const result = mstp(state, props);
    if (typeof result === "object") {
      return result;
    }
  }
  return {};
}

function mapDispatchToPropsCheck(mdtp: any, node: Node, props: any): Object {
  if (typeof mdtp === "function") {
    const result = mdtp(node.dispatch, props);
    if (typeof result === "object") {
      return result;
    }
  } else {
    if (typeof mdtp === "object") {
      let result = {};
      for (let key in mdtp) {
        if (mdtp.hasOwnProperty(key)) {
          let func = mdtp[key];
          if (typeof func === "function") {
            result[key] = function (payload) {
              node.dispatch(func(payload));
            };
          }
        }
      }
      return result;
    }
  }
  return {};
}
