use std::collections::HashMap;

use crate::Matrix;
use crate::matrix::{NewShape, Position};
use crate::matrix::BasicOperationsTrait;


#[test]
fn test_matrix_create()
{
    let m = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

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
    let m = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

    let expected = &NewShape(2, 2);

    assert_eq!(m.get_shape(), expected);
}


#[test]
fn test_get_elements()
{
    let m = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

    let expected = &HashMap::from([(Position(0, 0), 1.0), (Position(0, 1), 2.0), 
        (Position(1, 0), 3.0), (Position(1, 1), 4.0)]);

    assert_eq!(m.get_elements(), expected);
}


#[test]
fn test_get_mut_elements()
{
    let mut m = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

    let expected = &mut HashMap::from([(Position(0, 0), 1.0), (Position(0, 1), 2.0), 
        (Position(1, 0), 3.0), (Position(1, 1), 4.0)]);

    assert_eq!(m.get_mut_elements(), expected);
}


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
