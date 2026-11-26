class Greeter {
  constructor(name) {
    this.name = name;
  }

  greet() {
    return format(this.name);
  }
}

function format(name) {
  return `Hello ${name}`;
}

const shout = (text) => text.toUpperCase();

new Greeter("x").greet();
