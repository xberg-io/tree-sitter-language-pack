mod geometry {
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub trait Shape {
        fn area(&self) -> f64;
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }
    }
}

macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    let p = geometry::Point::new(1, 2);
    println!("{}", square!(p.x));
}
