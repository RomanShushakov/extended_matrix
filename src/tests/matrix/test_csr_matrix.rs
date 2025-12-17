#![allow(unused_imports)]

use crate::{BasicOperationsTrait, CsrMatrix, Position, SquareMatrix};

const ABS_TOL: f64 = 1e-12;

fn mat2x2(a11: f64, a12: f64, a21: f64, a22: f64) -> SquareMatrix<f64> {
    SquareMatrix::create(2, &[a11, a12, a21, a22])
}

#[test]
fn test_csr_from_square_matrix_and_spmv() {
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

#[test]
fn test_from_coo_basic_spmv() {
    // A = [4 1 0;
    //      1 3 1;
    //      0 1 2]
    let trip = vec![
        (0, 0, 4.0f64),
        (0, 1, 1.0),
        (1, 0, 1.0),
        (1, 1, 3.0),
        (1, 2, 1.0),
        (2, 1, 1.0),
        (2, 2, 2.0),
    ];

    let csr = CsrMatrix::from_coo(3, 3, &trip).unwrap();
    let y = csr.spmv(&[1.0, 2.0, 3.0]).unwrap();
    assert!((y[0] - 6.0).abs() < ABS_TOL);
    assert!((y[1] - 10.0).abs() < ABS_TOL);
    assert!((y[2] - 8.0).abs() < ABS_TOL);
}

#[test]
fn test_from_coo_sums_duplicates() {
    // A(0,0) = 1 + 2 = 3, A(1,1)=4
    let trip = vec![(0usize, 0usize, 1.0f64), (0, 0, 2.0), (1, 1, 4.0)];
    let csr = CsrMatrix::from_coo(2, 2, &trip).unwrap();
    let y = csr.spmv(&[1.0, 1.0]).unwrap();
    assert!((y[0] - 3.0).abs() < ABS_TOL);
    assert!((y[1] - 4.0).abs() < ABS_TOL);
}
