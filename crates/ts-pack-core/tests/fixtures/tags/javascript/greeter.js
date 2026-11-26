import { format as formatText } from "./format.js";

const DEFAULT_GREETING = "Hello";
const { languages, fallback } = config();
let greetCount = 0;
var legacyMode = false;
export const VERSION = "1.0";
export let activeGreeter = null;

class Base {}

/** Greets a person by name. */
class Greeter extends Base {
  name = "";
  static instances = 0;
  #secret = 1;

  constructor(name) {
    super();
    this.name = name;
  }

  greet() {
    return format(this.name);
  }

  #reset() {
    this.#secret = 0;
  }
}

const Shouter = class Loud extends Greeter {};

function format(name) {
  const local = `${DEFAULT_GREETING} ${name}`;
  return local;
}

function* names() {
  yield "a";
}

const shout = (text) => text.toUpperCase();

var whisper = function quiet(text) {
  return text.toLowerCase();
};

const api = {
  greet: (name) => new Greeter(name).greet(),
};

api.reset = () => {
  greetCount = 0;
};

function config() {
  return { languages: [], fallback: "en" };
}

new Greeter("x").greet();
