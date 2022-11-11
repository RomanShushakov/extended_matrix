#![allow(unused_imports)]

use std::collections::HashMap;

use crate::{SymmetricMatrix, Matrix};
use crate::matrix::{NewShape, Position, BasicOperationsTrait, IntoMatrixTrait};


#[test]
fn test_try_to_create()
{
    let m_1 = 
        SymmetricMatrix::try_to_create(2, &[1.0, 2.0, 3.0, 4.0], 0.0001);
    let m_2 = 
        SymmetricMatrix::try_to_create(2, &[1.0, -2.0, -2.00005, 4.0], 0.0001);

    let expected_m_2 = SymmetricMatrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 1.0), (Position(0, 1), -2.0), (Position(1, 0), -2.0), (Position(1, 1), 4.0)]
            ) 
        };

    assert_eq!(m_1, Err("Element [1, 0] does not match with [0, 1]!".to_string()));
    assert_eq!(m_2, Ok(expected_m_2));
}


#[test]
fn test_force_create()
{
    let mut warnings_1 = Vec::new();
    let m_1 = SymmetricMatrix::force_create(2, &[1.0, 2.0, 3.0, 4.0],
        0.0001, &mut warnings_1);
    let mut warnings_2 = Vec::new();
    let m_2 = SymmetricMatrix::force_create(2, &[1.0, -2.0, -2.00005, 4.0], 
        0.0001, &mut warnings_2);

    let expected_warnings_1 = vec!["Element [1, 0] does not match with [0, 1]!".to_string()];
    let expected_m_1 = SymmetricMatrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 1.0), (Position(0, 1), 2.0), (Position(1, 0), 2.0), (Position(1, 1), 4.0)]
            ) 
        };

    let expected_warnings_2: Vec<String> = Vec::new();
    let expected_m_2 = SymmetricMatrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 1.0), (Position(0, 1), -2.0), (Position(1, 0), -2.0), (Position(1, 1), 4.0)]
            ) 
        };

    assert_eq!(warnings_1, expected_warnings_1);
    assert_eq!(m_1, expected_m_1);
    assert_eq!(warnings_2, expected_warnings_2);
    assert_eq!(m_2, expected_m_2);

}


#[test]
fn test_get_shape()
{
    let m = SymmetricMatrix::try_to_create(2, &[1.0, 2.0, 2.0, 4.0], 
        0.0001).unwrap();

    let expected = &NewShape(2, 2);

    assert_eq!(m.get_shape(), expected);
}


#[test]
fn test_get_mut_shape()
{
    let mut m = SymmetricMatrix::try_to_create(2, &[1.0, 2.0, 2.000001, 4.0], 
        0.0001).unwrap();

    let expected = &mut NewShape(2, 2);

    assert_eq!(m.get_mut_shape(), expected);
}


#[test]
fn test_get_elements()
{
    let m = SymmetricMatrix::try_to_create(2, &[1.0, 2.0, 2.000001, 4.0], 
        0.0001).unwrap();

    let expected = &HashMap::from([(Position(0, 0), 1.0), (Position(0, 1), 2.0), 
        (Position(1, 0), 2.0), (Position(1, 1), 4.0)]);

    assert_eq!(m.get_elements(), expected);
}


#[test]
fn test_get_mut_elements()
{
    let mut m = SymmetricMatrix::try_to_create(2, &[1.0, 2.0, 2.000001, 4.0], 
        0.0001).unwrap();

    let expected = &mut HashMap::from([(Position(0, 0), 1.0), (Position(0, 1), 2.0), 
        (Position(1, 0), 2.0), (Position(1, 1), 4.0)]);

    assert_eq!(m.get_mut_elements(), expected);
}


#[test]
fn test_into_matrix()
{
    let m = SymmetricMatrix::try_to_create(2, &[1.0, 2.0, 2.000001, 4.0], 
        0.0001).unwrap();

    let expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 1.0), (Position(0, 1), 2.0), (Position(1, 0), 2.0), (Position(1, 1), 4.0)]
            ) 
        };

    assert_eq!(m.into_matrix(), expected);
}
