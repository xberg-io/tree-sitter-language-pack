import { log } from "./log";

export interface Shape {
  area(): number;
  readonly name: string;
}

export type Point = { x: number; y: number };

export enum Color {
  Red,
  Green,
}

export abstract class Base implements Shape {
  abstract area(): number;
  get name(): string {
    return "base";
  }
}

export class Circle extends Base {
  radius = 1;
  constructor(radius: number) {
    super();
    this.radius = radius;
  }
  area(): number {
    return Math.PI * this.radius ** 2;
  }
}

export namespace Geometry {
  export function unit(): Circle {
    return new Circle(1);
  }
}

declare module "shapes-ext" {
  export function extend(shape: Shape): void;
}

export const MAX_RADIUS = 10;

export const scale = (shape: Circle, factor: number): Circle => new Circle(shape.radius * factor);

function* ids(): Generator<number> {
  yield 1;
}

log(scale(Geometry.unit(), 2).area());
