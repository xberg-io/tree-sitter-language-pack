circuit Adder :
  module Half :
    input x : UInt<1>
    input y : UInt<1>
    output s : UInt<1>
    s <= xor(x, y)

  module Adder :
    input clock : Clock
    input a : UInt<8>
    input b : UInt<8>
    output sum : UInt<9>
    wire tmp : UInt<9>
    node total = add(a, b)
    inst half of Half
    half.x <= bits(a, 0, 0)
    half.y <= bits(b, 0, 0)
    tmp <= total
    sum <= tmp

  extmodule BlackBox :
    input in : UInt<1>
    defname = BlackBox
