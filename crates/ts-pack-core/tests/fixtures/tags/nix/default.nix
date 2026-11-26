{ pkgs, lib ? pkgs.lib }:
let
  version = "1.2.0";
  greet = name: "Hello ${name}";
  wrap = prefix: name: prefix + name;
  mkPackage = { pname, src }: pkgs.stdenv.mkDerivation {
    inherit pname src version;
    meta = { license = "mit"; };
  };
in
rec {
  hello = greet "world";
  shout = wrap "!" "x";
  meta.description = "demo";
  tools = {
    format = text: lib.toLower text;
    lint = "eslint";
  };
  app = mkPackage { pname = "app"; src = ./.; };
}
