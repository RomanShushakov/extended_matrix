#![allow(unused_imports)]

use std::collections::HashMap;

use crate::Matrix;
use crate::matrix::{NewShape, Position, BasicOperationsTrait, IntoMatrixTrait};


#[test]
fn test_matrix_create()
{
    let m = Matrix::create(2, 2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 1.0), (Position(0, 1), 2.0), (Position(1, 0), 3.0), (Position(1, 1), 4.0)]
            ) 
        };

    assert_eq!(m, expected);
}


#[test]
fn test_get_shape()
{
    let m = Matrix::create(2, 2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = &NewShape(2, 2);

    assert_eq!(m.get_shape(), expected);
}


#[test]
fn test_get_mut_shape()
{
    let mut m = Matrix::create(2, 2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = &mut NewShape(2, 2);

    assert_eq!(m.get_mut_shape(), expected);
}


#[test]
fn test_get_elements()
{
    let m = Matrix::create(2, 2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = &HashMap::from([(Position(0, 0), 1.0), (Position(0, 1), 2.0), 
        (Position(1, 0), 3.0), (Position(1, 1), 4.0)]);

    assert_eq!(m.get_elements(), expected);
}


#[test]
fn test_get_mut_elements()
{
    let mut m = Matrix::create(2, 2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = &mut HashMap::from([(Position(0, 0), 1.0), (Position(0, 1), 2.0), 
        (Position(1, 0), 3.0), (Position(1, 1), 4.0)]);

    assert_eq!(m.get_mut_elements(), expected);
}


#[test]
fn test_into_matrix()
{
    let m = Matrix::create(2, 2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 1.0), (Position(0, 1), 2.0), (Position(1, 0), 3.0), (Position(1, 1), 4.0)]
            ) 
        };

    assert_eq!(m.into_matrix(), expected);
}
