type shape =
  | Circle(float)
  | Square(float)

type point = {
  x: float,
  y: float,
}

let pi = 3.14

let area = shape =>
  switch shape {
  | Circle(r) => pi *. r *. r
  | Square(s) => s *. s
  }

module Utils = {
  let double = x => x * 2
  let factor = 2
}

@val external log: string => unit = "console.log"

exception NotFound

let () = log(Float.toString(area(Circle(1.0))))
