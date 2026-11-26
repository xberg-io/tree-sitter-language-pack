import Nat "mo:base/Nat";

module Utils {
  public func double(n : Nat) : Nat { n * 2 };
  public func quadruple(n : Nat) : Nat { double(double(n)) };
};

actor Counter {
  type Mode = { #up; #down };

  stable var count : Nat = 0;

  let step = 1;

  class Tracker(start : Nat) {
    public func current() : Nat { start };
  };

  public func increment() : async Nat {
    count += Utils.double(step);
    count
  };

  public query func get() : async Nat {
    count
  };
};
