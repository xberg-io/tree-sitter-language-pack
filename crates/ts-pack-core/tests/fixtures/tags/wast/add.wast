(module $math
  (global $counter (mut i32) (i32.const 0))
  (func $add (export "add") (param $a i32) (param $b i32) (result i32)
    (local $tmp i32)
    (local.set $tmp (i32.add (local.get $a) (local.get $b)))
    (local.get $tmp))
  (func $twice (param $x i32) (result i32)
    (call $add (local.get $x) (local.get $x))))

(assert_return (invoke "add" (i32.const 1) (i32.const 2)) (i32.const 3))
