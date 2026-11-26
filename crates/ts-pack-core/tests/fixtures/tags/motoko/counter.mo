import Debug "mo:base/Debug";

module Util {
  public func double(x : Nat) : Nat { x * 2 };
};

type Point = { x : Int; y : Int };

let origin : Point = { x = 0; y = 0 };

actor Main {
  stable var total : Nat = 0;

  class Counter(init : Nat) {
    var count = init;
    public func inc() { count += 1 };
  };

  func bump(step : Nat) : Nat {
    let next = total + step;
    next
  };

  public func greet(name : Text) : async Text {
    total += Util.double(bump(1));
    Debug.print(name);
    "Hello, " # name
  };
};
