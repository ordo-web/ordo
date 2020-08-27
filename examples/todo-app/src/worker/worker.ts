import("../../bin/pkg/todo_app").then((wasm) => {
  const myApp = new wasm.TodoApp();
});
