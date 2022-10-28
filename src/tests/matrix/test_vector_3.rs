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


#[test]
fn test_projection_perpendicular_to_vector()
{
    let v_1 = Vector3::create(&[0.0, 0.0, 1.0]);
    let v_2 = Vector3::create(&[12.0, 12.0, 12.0]);
    let mut v_3 = Vector3::create(&[-1.0, 1.0, 1.0]);
    v_3 = v_3.transpose();
    let v_4 = Vector3::create(&[10.0, 0.0, 0.0]);
    let v_5 = Vector3::create(&[2.0, 6.0, 8.0]);
    let mut v_6 = Vector3::create(&[3.0, -2.0, 10.0]);
    v_6 = v_6.transpose();
    let v_7 = Vector3::create(&[0.0, 0.0, 1.0]);
    let v_8 = Vector3::create(&[0.0, 0.0, 10.0]);

    let v_expected_1 = Vector3
        {
            shape: NewShape(3, 1),
            elements: HashMap::from([
                (Position(0, 0), -0.3333333333333333), 
                (Position(1, 0), -0.3333333333333333), 
                (Position(2, 0), 0.6666666666666666),
            ]) 
        };

    let v_expected_2 = Vector3
        {
            shape: NewShape(3, 1),
            elements: HashMap::from([(Position(0, 0), 0.0), (Position(1, 0), 1.0), (Position(2, 0), 1.0)]) 
        };

    let v_expected_3 = Vector3
        {
            shape: NewShape(3, 1),
            elements: HashMap::from([
                (Position(0, 0), 0.03539823008849559), 
                (Position(1, 0), 7.309734513274336), 
                (Position(2, 0), 1.4513274336283186),
            ]) 
        };

    let v_expected_4 = Vector3
        {
            shape: NewShape(3, 1),
            elements: HashMap::from([(Position(0, 0), 0.0), (Position(1, 0), 0.0), (Position(2, 0), 0.0)]) 
        };

    assert_eq!(v_1.projection_perpendicular_to_vector(&v_2), v_expected_1);
    assert_eq!(v_3.projection_perpendicular_to_vector(&v_4), v_expected_2);
    assert_eq!(v_5.projection_perpendicular_to_vector(&v_6), v_expected_3);
    assert_eq!(v_7.projection_perpendicular_to_vector(&v_8), v_expected_4);
}
