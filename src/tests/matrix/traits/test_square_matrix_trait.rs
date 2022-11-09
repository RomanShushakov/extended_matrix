#![allow(unused_imports)]

use crate::{SquareMatrix, SquareMatrixTrait, Vector, Vector3, BasicOperationsTrait};


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
        7.0f32, 6.0, 3.0, 7.0,
        3.0, 5.0, 7.0, 2.0, 
        5.0, 4.0, 3.0, 5.0,
        5.0, 6.0, 5.0, 4.0]);
    assert_eq!(sm_1.determinant(1e-6), -2.0);
    assert_eq!(sm_2.determinant(1e-6), 0.0);
    assert_eq!(sm_3.determinant(1e-6), 1.0);
    assert_eq!(sm_4.determinant(1e-6), -2.9999999999999982);
    assert_eq!(sm_5.determinant(1e-6), 8.0);
    assert_eq!(sm_6.determinant(1e-6), 100.0);
    assert_eq!(sm_7.determinant(1e-6), 89.99999999999986);
    assert_eq!(sm_8.determinant(1e-6), -9.999998);
}


#[test]
fn test_gauss_gep() -> Result<(), String>
{
    let a_1 = SquareMatrix::create(3, &[
        3.0, -0.1, -0.2,
        0.1, 7.0, -0.3,
        0.3, -0.2, 10.0,
    ]);
    let b_1 = Vector::create(&[7.85, -19.3, 71.4]);
    let mut x_1 = Vector3::create(&[0.0; 3]);

    let a_2 = SquareMatrix::create(4, &[
        5.0, -4.0, 1.0, 0.0,
        -4.0, 6.0, -4.0, 1.0,
        1.0, -4.0, 6.0, -4.0,
        0.0, 1.0, -4.0, 5.0,
    ]);
    let b_2 = Vector::create(&[0.0, 1.0, 0.0, 0.0]);
    let mut x_2 = Vector::create(&[0.0; 4]);

    let a_3 = SquareMatrix::create(5, &[
        2.0f32, -2.0, 0.0, 0.0, -1.0,
        -2.0, 3.0, -2.0, 0.0, 0.0,
        0.0, -2.0, 5.0, -3.0, 0.0,
        0.0, 0.0, -3.0, 10.0, 4.0,
        -1.0, 0.0, 0.0, 4.0, 10.0,
    ]);

    let mut b_3 = Vector::create(&[0.0, 1.0, 0.0, 0.0, 0.0]);
    b_3 = b_3.transpose();
    let mut x_3 = Vector::create(&[0.0; 5]);

    let expected_x_1 = Vector3::create(&[3.0, -2.5, 7.000000000000002]);
    let expected_x_2 = Vector::create(&[1.6000000000000028, 2.6000000000000045, 
        2.400000000000004, 1.4000000000000024]);
    let expected_x_3 = Vector::create(&[635.995, 618.9951, 291.99768, 73.99941, 33.999725]);

    a_1.gauss_gep(&b_1, &mut x_1, 1e-6)?;
    a_2.gauss_gep(&b_2, &mut x_2, 1e-6)?;
    a_3.gauss_gep(&b_3, &mut x_3, 1e-6)?;

    assert_eq!(x_1, expected_x_1);
    assert_eq!(x_2, expected_x_2);
    assert_eq!(x_3, expected_x_3);

    Ok(())
}
