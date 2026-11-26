(module
  (import "env" "log" (func $log (param i32)))
  (type $binop (func (param i32 i32) (result i32)))
  (memory $mem 1)
  (global $counter (mut i32) (i32.const 0))
  (table $fns 2 funcref)
  (func $add (type $binop) (param $a i32) (param $b i32) (result i32)
    (local $sum i32)
    local.get $a
    local.get $b
    i32.add
    local.set $sum
    local.get $sum
    call $log
    local.get $sum)
  (func $main (export "main")
    i32.const 1
    i32.const 2
    call $add
    drop))
