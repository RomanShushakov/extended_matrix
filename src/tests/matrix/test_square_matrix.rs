#![allow(unused_imports)]

use std::collections::HashMap;

use crate::{Matrix, SquareMatrix};
use crate::matrix::{Position, NewShape, BasicOperationsTrait, IntoMatrixTrait};


#[test]
fn test_square_matrix_create()
{
    let m = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);

    let expected = SquareMatrix 
        { 
            shape: crate::matrix::NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 1.0), (Position(0, 1), 2.0), (Position(1, 0), 3.0), (Position(1, 1), 4.0)]
            ) 
        };

    assert_eq!(m, expected);
}


#[test]
fn test_get_shape()
{
    let m = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);

    let expected = &NewShape(2, 2);

    assert_eq!(m.get_shape(), expected);
}


#[test]
fn test_get_mut_shape()
{
    let mut m = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);

    let expected = &mut NewShape(2, 2);

    assert_eq!(m.get_mut_shape(), expected);
}


#[test]
fn test_get_elements()
{
    let m = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);

    let expected = &HashMap::from([(Position(0, 0), 1.0), (Position(0, 1), 2.0), 
        (Position(1, 0), 3.0), (Position(1, 1), 4.0)]);

    assert_eq!(m.get_elements(), expected);
}


#[test]
fn test_get_mut_elements()
{
    let mut m = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);

    let expected = &mut HashMap::from([(Position(0, 0), 1.0), (Position(0, 1), 2.0), 
        (Position(1, 0), 3.0), (Position(1, 1), 4.0)]);

    assert_eq!(m.get_mut_elements(), expected);
}


#[test]
fn test_into_matrix()
{
    let m = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);

    let expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 1.0), (Position(0, 1), 2.0), (Position(1, 0), 3.0), (Position(1, 1), 4.0)]
            ) 
        };

    assert_eq!(m.into_matrix(), expected);
}
