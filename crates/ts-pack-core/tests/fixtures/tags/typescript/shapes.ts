import { log } from "./log";

const DEFAULT_RADIUS = 1;
const { PI, E } = Math;
let shapeCount = 0;
var legacyTotal = 0;
export let lastShape: Shape | undefined;

export interface Shape {
  area(): number;
  readonly name: string;
}

export interface Solid extends Shape, Comparable<Solid> {
  volume(): number;
}

interface Comparable<T> {
  compare(other: T): number;
  compare(other: T, strict: boolean): number;
}

export type Point = { x: number; y: number };

export enum Color {
  Red,
  Green = 2,
}

export abstract class Base implements Shape {
  abstract area(): number;
  get name(): string {
    return "base";
  }
}

export class Circle extends Base implements Comparable<Circle> {
  radius = DEFAULT_RADIUS;
  static #instances = 0;

  constructor(radius: number) {
    super();
    this.radius = radius;
  }

  area(): number {
    const local = this.radius ** 2;
    return PI * local;
  }

  compare(other: Circle): number;
  compare(other: Circle, strict: boolean): number;
  compare(other: Circle, strict = false): number {
    return strict ? 0 : this.area() - other.area();
  }
}

export namespace Geometry {
  export const ORIGIN = 0;

  export function unit(): Circle {
    return new Circle(1);
  }
}

declare module "shapes-ext" {
  export function extend(shape: Shape): void;
}

export const MAX_RADIUS = 10;

export const scale = (shape: Circle, factor: number): Circle => new Circle(shape.radius * factor);

export function describe(shape: Circle): string;
export function describe(shape: Shape): string;
export function describe(shape: Shape): string {
  return shape.name;
}

function parse(text: string): number;
function parse(text: number): number;
function parse(text: string | number): number {
  return Number(text);
}

var legacyArea = function (shape: Shape): number {
  return shape.area();
};

const registry = {
  register: (shape: Shape) => shape,
};

registry.reset = () => undefined;

function* ids(): Generator<number> {
  yield 1;
}

log(scale(Geometry.unit(), 2).area());
