#![allow(unused_imports)]

use crate::{SquareMatrix, SquareMatrixTrait};


#[test]
fn test_determinant()
{
    let sm_1 = SquareMatrix::create(2, &[
        1.0, 2.0, 
        3.0, 4.0]);
    let sm_2 = SquareMatrix::create(5, &[0.0; 25]);
    let sm_3 = SquareMatrix::create(2, &[
        1.0, 1e4, 
        1e-4, 2.0]);
    let sm_4 = SquareMatrix::create(3, &[
        1.0, 2.0, 3.0, 
        4.0, 5.0, 6.0,
        7.0,8.0, 10.0]);
    let sm_5 = SquareMatrix::create(3, &[
        2.0, 1.0, 0.0, 
        1.0, 3.0, 1.0, 
        0.0, 1.0, 2.0]);
    let sm_6 = SquareMatrix::create(4, &[
        1.0, 1.0, 3.0, 4.0, 
        2.0, 0.0, 0.0, 8.0, 
        3.0, 0.0, 0.0, 2.0, 
        4.0, 4.0, 7.0, 5.0]);
    let sm_7 = SquareMatrix::create(6, &[
        1.0, 2.0, 0.0, 0.0, 0.0, 0.0,
        3.0, 4.0, 0.0, 0.0, 0.0, 0.0,
        7.0, 6.0, 5.0, 4.0, 0.0, 0.0,
        2.0, 3.0, 4.0, 5.0, 0.0, 0.0,
        5.0, 1.0, 2.0, 6.0, 7.0, 3.0, 
        2.0, 7.0, 5.0, 3.0, 4.0, 1.0]);
    let sm_8 = SquareMatrix::create(4, &[
        7.0, 6.0, 3.0, 7.0,
        3.0, 5.0, 7.0, 2.0, 
        5.0, 4.0, 3.0, 5.0,
        5.0, 6.0, 5.0, 4.0]);
    assert_eq!(sm_1.determinant(1e-9), Ok(-2.0));
    assert_eq!(sm_2.determinant(1e-9), Ok(0.0));
    assert_eq!(sm_3.determinant(1e-9), Ok(1.0));
    assert_eq!(sm_4.determinant(1e-9), Ok(-2.9999999999999982));
    assert_eq!(sm_5.determinant(1e-9), Ok(8.0));
    assert_eq!(sm_6.determinant(1e-9), Ok(100.0));
    assert_eq!(sm_7.determinant(1e-9), Ok(89.99999999999986));
    assert_eq!(sm_8.determinant(1e-9), Ok(-9.999999999999998));
}
