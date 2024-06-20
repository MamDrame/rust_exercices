#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));
pub fn multiply(m: Matrix, multiplier: i32) -> Matrix {
    let (a, b) = m.0;
    let (c, d) = m.1;
    Matrix(
        (a * multiplier, b * multiplier),
        (c * multiplier, d * multiplier),
    )
}
