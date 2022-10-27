#![allow(unused_imports)]

use std::collections::HashMap;

use crate::{Matrix, Vector3, SquareMatrix};
use crate::matrix::{Position, NewShape, BasicOperationsTrait, IntoMatrixTrait, VectorTrait};


#[test]
fn test_vec_3_create()
{
    let v = Vector3::create(&[1.0, 2.0, 3.0]);

    let v_expected = Vector3
        { 
            shape: NewShape(3, 1), 
            elements: HashMap::from([(Position(0, 0), 1.0), (Position(1, 0), 2.0), (Position(2, 0), 3.0)]), 
        };

    assert_eq!(v, v_expected);
}


#[test]
fn test_get_shape()
{
    let v = Vector3::create(&[1.0, 2.0, 3.0]);

    let v_expected = &NewShape(3, 1);

    assert_eq!(v.get_shape(), v_expected);
}


#[test]
fn test_get_mut_shape()
{
    let mut v = Vector3::create(&[1.0, 2.0, 3.0]);

    let v_expected = &mut NewShape(3, 1);

    assert_eq!(v.get_mut_shape(), v_expected);
}


#[test]
fn test_get_elements()
{
    let v = Vector3::create(&[1.0, 2.0, 3.0]);

    let v_expected = &HashMap::from([
        (Position(0, 0), 1.0), (Position(1, 0), 2.0), (Position(2, 0), 3.0)]);

    assert_eq!(v.get_elements(), v_expected);
}


#[test]
fn test_get_mut_elements()
{
    let mut v = Vector3::create(&[1.0, 2.0, 3.0]);

    let v_expected = &mut HashMap::from([
        (Position(0, 0), 1.0), (Position(1, 0), 2.0), (Position(2, 0), 3.0)]);

    assert_eq!(v.get_mut_elements(), v_expected);
}


#[test]
fn test_into_matrix()
{
    let v = Vector3::create(&[1.0, 2.0, 3.0]);

    let m_expected = Matrix 
        { 
            shape: NewShape(3, 1), 
            elements: HashMap::from([(Position(0, 0), 1.0), (Position(1, 0), 2.0), (Position(2, 0), 3.0)]) 
        };

    assert_eq!(v.into_matrix(), m_expected);
}


#[test]
fn test_cross_product()
{
    let v_1 = Vector3::create(&[3.0, 3.0, 0.0]);
    let v_2 = Vector3::create(&[0.0, 2.0, 2.0]);
    let v_3 = v_1.transpose();
    let v_4 = v_2.transpose();

    let v_expected = Vector3 
        { 
            shape: NewShape(3, 1), 
            elements: HashMap::from([(Position(0, 0), 6.0), (Position(1, 0), -6.0), (Position(2, 0), 6.0)]) 
        };

    assert_eq!(v_1.cross_product(&v_2), v_expected.clone());
    assert_eq!(v_1.cross_product(&v_4), v_expected.clone());
    assert_eq!(v_3.cross_product(&v_2), v_expected.clone());
    assert_eq!(v_3.cross_product(&v_4), v_expected.clone());
}


#[test]
fn test_get_components()
{
    let v_1 = Vector3::create(&[3.0, 3.0, 0.0]);
    let v_2 = v_1.transpose();

    let expected = [3.0, 3.0, 0.0];

    assert_eq!(v_1.get_components(), expected.clone());
    assert_eq!(v_2.get_components(), expected);
}


#[test]
fn test_angle_between_vectors()
{
    let v_1 = Vector3::create(&[3.0, 3.0, 0.0]);
    let v_2 = Vector3::create(&[0.0, 2.0, 2.0]);
    let v_3 = v_1.transpose();
    let v_4 = v_2.transpose();

    let expected = 1.0471975511965979; 

    assert_eq!(v_1.angle_between_vectors(&v_2), expected);
    assert_eq!(v_1.angle_between_vectors(&v_4), expected);
    assert_eq!(v_2.angle_between_vectors(&v_3), expected);
}
