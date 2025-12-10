#![allow(unused_imports)]

use crate::{BasicOperationsTrait, CsrMatrix, Position, SquareMatrix};

fn mat2x2(a11: f64, a12: f64, a21: f64, a22: f64) -> SquareMatrix<f64> {
    SquareMatrix::create(2, &[a11, a12, a21, a22])
}

#[test]
fn test_csr_from_square_matrix_and_spmv() {
    const ABS_TOL: f64 = 1e-12;

    // A = [[4, 0],
    //      [1, 3]]
    let a = mat2x2(4.0, 0.0, 1.0, 3.0);
    let csr = CsrMatrix::from_square_matrix(&a).unwrap();

    // x = [1, 2]^T
    let x = vec![1.0_f64, 2.0_f64];
    let y = csr.spmv(&x).unwrap();

    // y = A x = [4*1 + 0*2, 1*1 + 3*2] = [4, 7]
    assert!((y[0] - 4.0).abs() < ABS_TOL);
    assert!((y[1] - 7.0).abs() < ABS_TOL);
}
