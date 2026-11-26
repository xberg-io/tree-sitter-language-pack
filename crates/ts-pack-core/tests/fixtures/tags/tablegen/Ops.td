include "llvm/Target/Target.td"

defvar Width = 32;

class Reg<string n, int num> {
  string Name = n;
  int Num = num;
}

def R0 : Reg<"r0", 0>;
def R1 : Reg<"r1", 1>;

multiclass ALU<string opc> {
  def rr : Reg<opc, 2>;
}

defm ADD : ALU<"add">;

let Num = 7 in
def SP : Reg<"sp", 7>;

deftype RegList = list<Reg>;

defset list<Reg> GPRs = {
  def R2 : Reg<"r2", 2>;
}
