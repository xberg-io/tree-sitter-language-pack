#map = affine_map<(d0) -> (d0 + 1)>

module @kernels {
  func.func private @log(i32)

  func.func @add(%a: i32, %b: i32) -> i32 {
    %sum = arith.addi %a, %b : i32
    func.call @log(%sum) : (i32) -> ()
    return %sum : i32
  }

  func.func @main() -> i32 {
    %c1 = arith.constant 1 : i32
    %r = call @add(%c1, %c1) : (i32, i32) -> i32
    return %r : i32
  }
}
