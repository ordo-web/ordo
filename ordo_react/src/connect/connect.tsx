import * as React from "react";
import { Node } from "ordo_adapter";
import * as PropTypes from "prop-types";

export function connect(
  mapStateToProps?: null | ((state: Object, ownProps?: any) => Object),
  mapDispatchToProps?:
    | null
    | Object
    | ((dispatch: Object, ownProps?: any) => Object)
) {
  return function (WrappedComponent: React.ComponentType) {
    return class HOC extends React.Component<any, any> {
      constructor(props) {
        super(props);
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
        console.log("Hey!");
        this.forceUpdate();
      }

      render() {
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
            /**actualMapDispatchToProps = {
              dispatch: node.dispatch,
            };*/
            actualMapDispatchToProps = {};
          } else {
            // Parse single value and multi value JSON
            //console.log("JSON");
            //console.log(mapDispatchToProps);
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

        console.log("baumi");
        console.log(actualMapStateToProps, actualMapDispatchToProps);

        return (
          <WrappedComponent
            {...this.props}
            {...actualMapStateToProps}
            {...actualMapDispatchToProps}
          />
        );
      }
    };
  };
}
