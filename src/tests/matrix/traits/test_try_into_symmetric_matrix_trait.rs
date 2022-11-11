#![allow(unused_imports)]

use crate::{SquareMatrix, SymmetricMatrix, TryIntoSymmetricMatrixTrait};


#[test]
fn test_try_into_symmetric_matrix()
{
    let sm_1 = SquareMatrix::create(2, &[1.0, 2.0, 2.00000001, 4.0]);
    let sm_2 = SquareMatrix::create(2, &[1.0, 2.0, 3.0, 4.0]);

    let expected_sm_1 = SymmetricMatrix::try_to_create(2, 
        &[1.0, 2.0, 2.0, 4.0], 1e-6).unwrap();
    
    assert_eq!(sm_1.try_into_symmetric_matrix(1e-6), Ok(expected_sm_1));
    assert_eq!(sm_2.try_into_symmetric_matrix(1e-6), Err("Element [1, 0] does not match with [0, 1]!".to_string()));
}


#[test]
fn test_forced_into_symmetric_matrix()
{
    let mut warnings_1 = Vec::new();
    let sm_1 = SquareMatrix::create(2, &[1.0, 2.0, 2.00000001, 4.0]);
    let mut warnings_2 = Vec::new();
    let sm_2 = SquareMatrix::create(2, &[1.0, 2.0, 3.0, 4.0]);

    let expected_warnings_1: Vec<String> = Vec::new();
    let expected_sm_1 = SymmetricMatrix::try_to_create(2, 
        &[1.0, 2.0, 2.0, 4.0], 1e-6).unwrap();
    let expected_warnings_2 = vec!["Element [1, 0] does not match with [0, 1]!".to_string()];
    let expected_sm_2 = SymmetricMatrix::try_to_create(2, 
        &[1.0, 2.0, 2.0, 4.0], 1e-6).unwrap();
    
    assert_eq!(sm_1.forced_into_symmetric_matrix(1e-6, &mut warnings_1), expected_sm_1);
    assert_eq!(warnings_1, expected_warnings_1);
    assert_eq!(sm_2.forced_into_symmetric_matrix(1e-6, &mut warnings_2), expected_sm_2);
    assert_eq!(warnings_2, expected_warnings_2);
}
