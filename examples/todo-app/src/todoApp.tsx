import * as React from "react";
import AddTodo from "./components/addTodo";
import TodoList from "./components/todoList";
import VisibilityFilters from "./components/visibilityFilters";
import "style.css";

export default function TodoApp() {
  return (
    <div className="todo-app">
      <h1>Todo List</h1>
      <AddTodo />
      <TodoList />
      <VisibilityFilters />
    </div>
  );
}
