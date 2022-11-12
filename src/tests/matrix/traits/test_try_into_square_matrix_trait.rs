#![allow(unused_imports)]

use crate::{Matrix, SquareMatrix, TryIntoSquareMatrixTrait};


#[test]
fn test_try_into_square_matrix()
{
    let m_1 = Matrix::create(2, 2, &[1.0, 2.0, 3.0, 4.0]);
    let m_2 = Matrix::create(3, 1, &[1.0, 2.0, 3.0]);

    let expected_sm_1 = SquareMatrix::create(2, &[1.0, 2.0, 3.0, 4.0]);
    
    assert_eq!(m_1.try_into_square_matrix(), Ok(expected_sm_1));
    assert_eq!(m_2.try_into_square_matrix(), Err("Could not be converted into square matrix!".to_string()));
}
