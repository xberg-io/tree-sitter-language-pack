%struct.Point = type { i32, i32 }

@counter = global i32 0, align 4
@total = alias i32, ptr @counter
@.str = private constant [4 x i8] c"%d\0A\00"

declare i32 @printf(ptr, ...)

define i32 @add(i32 %a, i32 %b) {
entry:
  %sum = add i32 %a, %b
  ret i32 %sum
}

define i32 @main() {
entry:
  %r = call i32 @add(i32 1, i32 2)
  %p = call i32 (ptr, ...) @printf(ptr @.str, i32 %r)
  ret i32 0
}
