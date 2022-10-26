#![allow(unused_imports)]

use std::collections::HashMap;

use crate::{Matrix, SquareMatrix};
use crate::matrix::{NewShape, Position};
use crate::matrix::BasicOperationsTrait;


#[test]
#[should_panic(expected = "Element is absent")]
fn test_get_element_value()
{
    let m = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let sm = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);

    assert_eq!(m.get_element_value(&Position(0, 0)), &1.0);
    assert_eq!(sm.get_element_value(&Position(1, 1)), &4.0);
    assert_eq!(m.get_element_value(&Position(5, 0)), &2.0);
}


#[test]
#[should_panic(expected = "Element is absent")]
fn test_get_mut_element_value()
{
    let mut m = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let mut sm = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);

    assert_eq!(m.get_mut_element_value(&Position(0, 0)), &mut 1.0);
    assert_eq!(sm.get_mut_element_value(&Position(1, 1)), &mut 4.0);
    assert_eq!(m.get_mut_element_value(&Position(5, 0)), &mut 2.0);
}


#[test]
fn test_add()
{
    let m_1 = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m_2 = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m_3 = Matrix::create(2, 3, 
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let sm_1 = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);
    let sm_2 = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);

    let m_expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 2.0), (Position(0, 1), 4.0), (Position(1, 0), 6.0), (Position(1, 1), 8.0)]
            ) 
        };

    let sm_expected = SquareMatrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 2.0), (Position(0, 1), 4.0), (Position(1, 0), 6.0), (Position(1, 1), 8.0)]
            ) 
        };

    assert_eq!(m_1.add(&m_2), Ok(m_expected.clone()));
    assert_eq!(sm_1.add(&sm_2), Ok(sm_expected.clone()));
    assert_eq!(m_1.add(&sm_2), Ok(m_expected));
    assert_eq!(sm_1.add(&m_2), Ok(sm_expected));
    assert_eq!(m_1.add(&m_3), Err("Shapes of matrices do not conform to each other!".to_string()));
}


#[test]
fn test_subtract()
{
    let m_1 = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m_2 = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let m_3 = Matrix::create(2, 3, 
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let sm_1 = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);
    let sm_2 = SquareMatrix::create(2, vec![1.0, 2.0, 3.0, 4.0]);

    let m_expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 0.0), (Position(0, 1), 0.0), (Position(1, 0), 0.0), (Position(1, 1), 0.0)]
            ) 
        };

    let sm_expected = SquareMatrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 0.0), (Position(0, 1), 0.0), (Position(1, 0), 0.0), (Position(1, 1), 0.0)]
            ) 
        };

    assert_eq!(m_1.subtract(&m_2), Ok(m_expected.clone()));
    assert_eq!(sm_1.subtract(&sm_2), Ok(sm_expected.clone()));
    assert_eq!(m_1.subtract(&sm_2), Ok(m_expected));
    assert_eq!(sm_1.subtract(&m_2), Ok(sm_expected));
    assert_eq!(m_1.subtract(&m_3), Err("Shapes of matrices do not conform to each other!".to_string()));
}


#[test]
fn test_multiply_by_scalar()
{
    let m = Matrix::create(2, 2, vec![1.0, -2.0, 3.0, -4.0]);
    let sm = SquareMatrix::create(2, vec![1.0, -2.0, 3.0, -4.0]);

    let m_expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 5.0), (Position(0, 1), -10.0), (Position(1, 0), 15.0), (Position(1, 1), -20.0)]
            ) 
        };

    let sm_expected = SquareMatrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [(Position(0, 0), 5.0), (Position(0, 1), -10.0), (Position(1, 0), 15.0), (Position(1, 1), -20.0)]
            ) 
        };

    assert_eq!(m.multiply_by_scalar(5.0), m_expected);
    assert_eq!(sm.multiply_by_scalar(5.0), sm_expected);
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
    let sm_1 = SquareMatrix::create(3,
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    let sm_2 = SquareMatrix::create(3,
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let m_expected_1 = Matrix 
        { 
            shape: NewShape(1, 1), 
            elements: HashMap::from([(Position(0, 0), 32.0)]) 
        };

    let m_expected_2 = Matrix 
        { 
            shape: NewShape(2, 3), 
            elements: HashMap::from(
                [
                    (Position(0, 0), 30.0), (Position(0, 1), 36.0), (Position(0, 2), 42.0),
                    (Position(1, 0), 66.0), (Position(1, 1), 81.0), (Position(1, 2), 96.0),
                ],
            ) 
        };

    let m_expected_3 = Matrix 
        { 
            shape: NewShape(3, 3), 
            elements: HashMap::from(
                [
                    (Position(0, 0), 30.0), (Position(0, 1), 36.0), (Position(0, 2), 42.0),
                    (Position(1, 0), 66.0), (Position(1, 1), 81.0), (Position(1, 2), 96.0),
                    (Position(2, 0), 102.0), (Position(2, 1), 126.0), (Position(2, 2), 150.0),
                ],
            ) 
        };

    let m_expected_4 = Matrix 
        { 
            shape: NewShape(1, 3), 
            elements: HashMap::from([(Position(0, 0), 30.0), (Position(0, 1), 36.0), (Position(0, 2), 42.0)]) 
        };

    let m_expected_5 = Matrix 
        { 
            shape: NewShape(3, 1), 
            elements: HashMap::from([(Position(0, 0), 32.0), (Position(1, 0), 77.0), (Position(2, 0), 122.0)]) 
        };

    assert_eq!(m_1.multiply(&m_2), Ok(m_expected_1));
    assert_eq!(m_3.multiply(&m_4), Ok(m_expected_2));
    assert_eq!(sm_1.multiply(&sm_2), Ok(m_expected_3));
    assert_eq!(m_1.multiply(&sm_2), Ok(m_expected_4));
    assert_eq!(sm_1.multiply(&m_2), Ok(m_expected_5));
    assert_eq!(m_1.multiply(&m_5), Err("Shapes of matrices do not conform to each other!".to_string()));
}


#[test]
fn test_transpose()
{
    let m = Matrix::create(2, 3, 
        vec![1.0, -2.0, 3.0, -4.0, 5.0, -6.0]);
    let sm = SquareMatrix::create(2, vec![1.0, -2.0, 3.0, -4.0]);

    let m_expected = Matrix 
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

    let sm_expected = SquareMatrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from(
                [
                    (Position(0, 0), 1.0), (Position(0, 1), 3.0),
                    (Position(1, 0), -2.0), (Position(1, 1), -4.0),
                ]
            ) 
        };

    assert_eq!(m.transpose(), m_expected);
    assert_eq!(sm.transpose(), sm_expected);
}
