#![allow(unused_imports)]

use std::collections::HashMap;

use crate::Matrix;
use crate::matrix::{NewShape, Position};
use crate::matrix::BasicOperationsTrait;


#[test]
#[should_panic(expected = "Element is absent")]
fn test_get_element_value()
{
    let m = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

    assert_eq!(m.get_element_value(&Position(0, 0)), &1.0);
    assert_eq!(m.get_element_value(&Position(5, 0)), &2.0);
}


#[test]
#[should_panic(expected = "Element is absent")]
fn test_get_mut_element_value()
{
    let mut m = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

    assert_eq!(m.get_mut_element_value(&Position(0, 0)), &mut 1.0);
    assert_eq!(m.get_mut_element_value(&Position(5, 0)), &mut 2.0);
}


#[test]
fn test_add()
{
    let m_1 = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m_2 = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m_3 = Matrix::create(2, 3, 
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 2.0), (Position(0, 1), 4.0), (Position(1, 0), 6.0), (Position(1, 1), 8.0)]
            ) 
        };

    assert_eq!(m_1.add(&m_2), Ok(expected));
    assert_eq!(m_1.add(&m_3), Err("Shapes of matrices do not conform to each other!".to_string()));
}


#[test]
fn test_subtract()
{
    let m_1 = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m_2 = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m_3 = Matrix::create(2, 3, 
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 0.0), (Position(0, 1), 0.0), (Position(1, 0), 0.0), (Position(1, 1), 0.0)]
            ) 
        };

    assert_eq!(m_1.subtract(&m_2), Ok(expected));
    assert_eq!(m_1.subtract(&m_3), Err("Shapes of matrices do not conform to each other!".to_string()));
}


#[test]
fn test_multiply_by_scalar()
{
    let m = Matrix::create(2, 2, vec![1.0, -2.0, 3.0, -4.0]);

    let expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 5.0), (Position(0, 1), -10.0), (Position(1, 0), 15.0), (Position(1, 1), -20.0)]
            ) 
        };

    assert_eq!(m.multiply_by_scalar(5.0), expected);
}


#[test]
fn test_multiply()
{
    let m_1 = Matrix::create(1, 3, vec![1.0, 2.0, 3.0]);
    let m_2 = Matrix::create(3, 1, vec![4.0, 5.0, 6.0]);
    let m_3 = Matrix::create(2, 3, 
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let m_4 = Matrix::create(3, 3, 
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    let m_5 = Matrix::create(2, 3, 
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let expected_1 = Matrix 
        { 
            shape: NewShape(1, 1), 
            elements: HashMap::from([(Position(0, 0), 32.0)]) 
        };

    let expected_2 = Matrix 
        { 
            shape: NewShape(2, 3), 
            elements: HashMap::from(
                [
                    (Position(0, 0), 30.0), (Position(0, 1), 36.0), (Position(0, 2), 42.0),
                    (Position(1, 0), 66.0), (Position(1, 1), 81.0), (Position(1, 2), 96.0),
                ],
            ) 
        };

    assert_eq!(m_1.multiply(&m_2), Ok(expected_1));
    assert_eq!(m_3.multiply(&m_4), Ok(expected_2));
    assert_eq!(m_1.multiply(&m_5), Err("Shapes of matrices do not conform to each other!".to_string()));
}


#[test]
fn test_transpose()
{
    let m = Matrix::create(2, 3, 
        vec![1.0, -2.0, 3.0, -4.0, 5.0, -6.0]);

    let expected = Matrix 
        { 
            shape: NewShape(3, 2), 
            elements: HashMap::from(
                [
                    (Position(0, 0), 1.0), (Position(0, 1), -4.0),
                    (Position(1, 0), -2.0), (Position(1, 1), 5.0),
                    (Position(2, 0), 3.0), (Position(2, 1), -6.0),
                ]
            ) 
        };

    assert_eq!(m.transpose(), expected);
}
