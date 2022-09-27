use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i32, i32);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{};{}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f32,
    y: f32,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}; y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex { real: f32, imag: f32 }

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} + {:.1}i", self.real, self.imag)
    }
}

fn main() {
    let ss = Structure(42);
    println!("My little structure: {}", ss);

    let mm = MinMax(42, 24);
    println!("MinMax Display: {}", mm);
    println!("MinMax Debug: {:?}", mm);

    let point = Point2D {x: 1.0, y: 2.0};
    println!("Point Display: {point}");
    println!("Point Debug: {point:#?}");

    let c = Complex { real: 3.3, imag: 7.2 };
    println!("Display {c}");
    println!("Debug: {c:#?}");
}
