{ pkgs }:
let
  greet = name: "Hello ${name}";
  wrap = prefix: name: prefix + name;
in
{
  hello = greet "world";
  shout = wrap "!" "x";
}
