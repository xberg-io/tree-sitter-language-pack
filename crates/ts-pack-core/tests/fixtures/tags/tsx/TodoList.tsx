import { useState } from "react";

const PAGE_SIZE = 20;
let renderCount = 0;
export let lastFilter: Filter = "all";

export interface TodoProps {
  title: string;
  done: boolean;
}

interface Sortable<T> {
  sortKey(): string;
}

export interface Todo extends TodoProps, Sortable<Todo> {
  id: number;
}

export type Filter = "all" | "open";

export enum Priority {
  Low,
  High = 10,
}

export namespace Todos {
  export const EMPTY: TodoProps[] = [];
}

export function TodoItem({ title, done }: TodoProps) {
  return <li className={done ? "done" : ""}>{title}</li>;
}

export const TodoList = ({ items }: { items: TodoProps[] }) => {
  const [filter, setFilter] = useState<Filter>("all");
  return (
    <ul onClick={() => setFilter("open")}>
      {items.map((item) => (
        <TodoItem key={item.title} {...item} />
      ))}
    </ul>
  );
};

var legacyRender = function (props: TodoProps) {
  return <Panel>{props.title}</Panel>;
};

export function label(todo: Todo): string;
export function label(todo: TodoProps): string;
export function label(todo: TodoProps): string {
  return todo.title;
}

abstract class BaseStore {
  abstract size(): number;
}

export class Store extends BaseStore implements Sortable<Store> {
  items: TodoProps[] = [];

  constructor(initial: TodoProps[]) {
    super();
    this.items = initial;
  }

  size(): number {
    return this.items.length;
  }

  sortKey(): string {
    return "store";
  }

  add(item: TodoProps): void {
    console.log(item.title);
  }
}

const handlers = {
  onReset: () => new Store([]),
};

handlers.onClear = () => undefined;

function Panel({ children }: { children: string }) {
  renderCount += 1;
  return <section>{children}</section>;
}
