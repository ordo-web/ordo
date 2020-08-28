import * as React from "react";
import { Node } from "ordo_adapter";
import * as PropTypes from "prop-types";
import { isEqual } from "lodash";

/**mapStateToProps?: null | ((state: Object, ownProps?: any) => Object),
 mapDispatchToProps?:
 | null
 | Object
 | ((dispatch: Object, ownProps?: any) => Object)*/
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
        /**
         const node: Node = this.context.node;
         const state = node.getState();
        let actualMapStateToProps: Object;
        if (typeof mapStateToProps !== "function") {
          actualMapStateToProps = {};
        } else {
          actualMapStateToProps = mapStateToProps(state, this.props);
        }

        let actualMapDispatchToProps: Object;
        if (typeof mapDispatchToProps !== "function") {
          if (mapDispatchToProps === null || mapDispatchToProps === undefined) {
            actualMapDispatchToProps = {
              dispatch: node.dispatch,
            };
            actualMapDispatchToProps = {};
          } else {
            // Parse single value and multi value JSON
            actualMapDispatchToProps = {};
            for (let key in mapDispatchToProps) {
              let func = mapDispatchToProps[key];
              actualMapDispatchToProps[key] = function (payload) {
                node.dispatch(func(payload));
              };
            }
          }
        } else {
          actualMapDispatchToProps = mapDispatchToProps(
            node.dispatch,
            this.props
          );
        }

        //console.log("baumi");
        //console.log(actualMapStateToProps, actualMapDispatchToProps);

        return (
          <WrappedComponent
            {...this.props}
            {...actualMapStateToProps}
            {...actualMapDispatchToProps}
          />
        );*/
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
