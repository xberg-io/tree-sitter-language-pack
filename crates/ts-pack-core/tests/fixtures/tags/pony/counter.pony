use "collections"

trait Named
  fun name(): String

interface Countable is Named
  fun count(): USize

primitive Limits
  fun max_count(): USize => 10

struct Point
  var x: F64 = 0
  var y: F64 = 0

  new create(x': F64, y': F64) =>
    x = x'
    y = y'

class Counter is (Named & Countable)
  var _count: USize = 0

  new create() =>
    _count = 0

  fun name(): String => "counter"

  fun count(): USize => _count

  fun ref increment() =>
    _count = (_count + 1).min(Limits.max_count())

actor Main
  let _env: Env

  new create(env: Env) =>
    _env = env
    let counter = Counter
    counter.increment()
    report(counter.count())

  be report(value: USize) =>
    _env.out.print(value.string())

type Size is USize
