/**
 * Finds calls to functions named `target`.
 */

import javascript

module Targets {
  newtype TKind =
    TDirect() or
    TIndirect(int depth) { depth in [1 .. 3] }

  class Callee extends Function {
    Callee() { this.getName() = "target" }

    string describe() { result = this.getName() }
  }

  predicate isTarget(Function f) { f instanceof Callee }

  predicate hasTargets() { exists(Callee c | isTarget(c)) }
}

from Targets::Callee c
where Targets::hasTargets()
select c, c.describe()
