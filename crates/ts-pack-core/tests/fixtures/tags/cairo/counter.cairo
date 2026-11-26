use core::traits::Into;

const MAX_COUNT: u32 = 100;

#[derive(Drop, Copy)]
struct Point {
    x: u32,
    y: u32,
}

enum Direction {
    Up: (),
    Down: (),
}

trait Shape<T> {
    fn area(self: @T) -> u32;
}

impl PointShape of Shape<Point> {
    fn area(self: @Point) -> u32 {
        *self.x * *self.y
    }
}

mod helpers {
    fn double(value: u32) -> u32 {
        value * 2
    }
}

fn main() -> u32 {
    let p = Point { x: 2, y: 3 };
    helpers::double(p.area())
}
