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
      unsubscribe: any;
      constructor(props) {
        super(props);
      }

      static contextTypes = {
        node: PropTypes.object,
      };

      componentDidMount() {
        this.unsubscribe = this.context.node.subscribe(this.handleChange);
      }

      componentWillUnmount() {
        this.unsubscribe();
      }

      handleChange() {
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
            actualMapDispatchToProps = node.dispatch;
          } else {
            // Parse single value and multi value JSON
            //console.log("JSON");
            //console.log(mapDispatchToProps);
            for (let key in mapDispatchToProps) {
              let func = mapDispatchToProps[key];
              mapDispatchToProps[key] = (payload) => {
                node.dispatch(func(payload));
              };
            }
            actualMapDispatchToProps = mapDispatchToProps;
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
