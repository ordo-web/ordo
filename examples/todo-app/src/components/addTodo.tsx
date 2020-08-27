import * as React from "react";
import { connect } from "react-redux";
import { addTodo } from "../../bin/ordo-bindings/TodoAction.js";
//import {TodoEntry} from "../../bin/pkg";
//import { TodoEntry } from "../../bin/pkg";

class AddTodo extends React.Component<any, any> {
  constructor(props) {
    super(props);
    this.state = { input: "", nextTodoId: 0 };
  }

  updateInput = (input) => {
    this.setState({ input });
  };

  handleAddTodo = () => {
    const nextTodoId = this.state.nextTodoId + 1;
    this.props.addTodo({
      id: this.state.nextTodoId,
      content: this.state.input,
    });
    this.setState({ input: "", nextTodoId: nextTodoId });
  };

  render() {
    return (
      <div>
        <input
          onChange={(e) => this.updateInput(e.target.value)}
          value={this.state.input}
        />
        <button className="add-todo" onClick={this.handleAddTodo}>
          Add Todo
        </button>
      </div>
    );
  }
}

export default connect(null, { addTodo })(AddTodo);
// export default AddTodo;
