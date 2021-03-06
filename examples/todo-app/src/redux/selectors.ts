import { VISIBILITY_FILTERS } from "../constants";

export const getTodosState = (store) => store.todos;

export const getTodoList = (store) =>
  getTodosState(store) ? getTodosState(store).all_ids : [];

export const getTodoById = (store, id) =>
  getTodosState(store) ? { ...getTodosState(store).by_ids[id], id } : {};

/**
 * example of a slightly more complex selector
 * select from store combining information from multiple reducers
 */
export const getTodos = (store) =>
  getTodoList(store).map((id) => getTodoById(store, id));

export const getTodosByVisibilityFilter = (store, visibilityFilter) => {
  const allTodos = getTodos(store);
  switch (visibilityFilter.filter) {
    case VISIBILITY_FILTERS.COMPLETED:
      return allTodos.filter((todo) => todo.completed);
    case VISIBILITY_FILTERS.INCOMPLETE:
      return allTodos.filter((todo) => !todo.completed);
    case VISIBILITY_FILTERS.ALL:
    default:
      return allTodos;
  }
};
