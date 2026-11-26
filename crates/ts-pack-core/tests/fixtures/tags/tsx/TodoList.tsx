import { useState } from "react";

export interface TodoProps {
  title: string;
  done: boolean;
}

export type Filter = "all" | "open";

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

export class Store {
  add(item: TodoProps): void {
    console.log(item.title);
  }
}
