use std::fmt;

pub const MAX_SIDES: u32 = 8;

static mut COUNTER: u32 = 0;

pub type Id = u64;

mod geometry {
    #[derive(Debug, Clone)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub struct Meters(pub f64);

    pub enum Color {
        Red,
        Rgb(u8, u8, u8),
    }

    pub union Bits {
        int: u32,
        float: f32,
    }

    pub trait Shape {
        const SIDES: u32;
        type Unit;

        fn area(&self) -> f64;

        fn describe(&self) -> String {
            String::from("shape")
        }
    }

    impl Point {
        pub const ORIGIN: Point = Point { x: 0, y: 0 };

        pub fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }
    }

    impl Shape for Point {
        const SIDES: u32 = 0;
        type Unit = i32;

        fn area(&self) -> f64 {
            0.0
        }
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

impl fmt::Display for geometry::Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = geometry::Point::new(1, 2);
    const LOCAL: i32 = 3;
    let n = unsafe { abs(-LOCAL) };
    println!("{} {}", square!(p.x), n);
    p.area();
}
