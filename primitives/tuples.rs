fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_p, bool_p) = pair;
    (bool_p, int_p)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "( {}, {} )", self.0, self.1)?;
        write!(f, "( {}, {} )", self.2, self.3)
    }
}

fn transpose(m: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = m;
    Matrix(a, c, b, d)
}

fn main() {
    let reversed_pair = reverse((3, true));
    println!("{reversed_pair:?}");

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
