use std::fmt;

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n({:.2} {:.2})\n({:.2} {:.2})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let m = matrix;
    Matrix (m.0, m.2, m.1, m.3)
}

fn main() {
    let m = Matrix(1.0, 2.0, 1.1, 2.2);
    println!("m={}", m);
    println!("transpose(m)={}", transpose(m));
    // assert!(m==m, "m==m");
}


