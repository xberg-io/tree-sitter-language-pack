import javascript

class Callee extends Function {
  Callee() { this.getName() = "target" }

  string describe() { result = this.getName() }
}

predicate isTarget(Function f) { f instanceof Callee }

from Callee c
select c, c.describe()
