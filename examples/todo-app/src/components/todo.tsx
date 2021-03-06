import * as React from "react";
import { connect } from "ordo-react";
import cx from "classnames";
import { toggleTodo } from "../../bin/ordo-bindings/TodoAction.js";

const Todo = ({ todo, toggleTodo }) => (
  <li className="todo-item" onClick={() => toggleTodo(todo.id)}>
    {todo && todo.completed ? "👌" : "👋"}{" "}
    <span
      className={cx(
        "todo-item__text",
        todo && todo.completed && "todo-item__text--completed"
      )}
    >
      {todo.content}
    </span>
  </li>
);

// export default Todo;
export default connect(null, { toggleTodo })(Todo);
