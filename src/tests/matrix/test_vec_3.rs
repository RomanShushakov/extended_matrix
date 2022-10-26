#![allow(unused_imports)]

use std::collections::HashMap;

use crate::{Matrix, Vec3};
use crate::matrix::{Position, NewShape, BasicOperationsTrait, IntoMatrixTrait};


#[test]
fn test_vec_3_create()
{
    let v = Vec3::create(&[1.0, 2.0, 3.0]);

    let v_expected = Vec3
        { 
            shape: NewShape(3, 1), 
            elements: HashMap::from([(Position(0, 0), 1.0), (Position(1, 0), 2.0), (Position(2, 0), 3.0)]), 
        };

    assert_eq!(v, v_expected);
}


#[test]
fn test_get_shape()
{
    let v = Vec3::create(&[1.0, 2.0, 3.0]);

    let v_expected = &NewShape(3, 1);

    assert_eq!(v.get_shape(), v_expected);
}


#[test]
fn test_get_mut_shape()
{
    let mut v = Vec3::create(&[1.0, 2.0, 3.0]);

    let v_expected = &mut NewShape(3, 1);

    assert_eq!(v.get_mut_shape(), v_expected);
}


#[test]
fn test_get_elements()
{
    let v = Vec3::create(&[1.0, 2.0, 3.0]);

    let v_expected = &HashMap::from([
        (Position(0, 0), 1.0), (Position(1, 0), 2.0), (Position(2, 0), 3.0)]);

    assert_eq!(v.get_elements(), v_expected);
}


#[test]
fn test_get_mut_elements()
{
    let mut v = Vec3::create(&[1.0, 2.0, 3.0]);

    let v_expected = &mut HashMap::from([
        (Position(0, 0), 1.0), (Position(1, 0), 2.0), (Position(2, 0), 3.0)]);

    assert_eq!(v.get_mut_elements(), v_expected);
}


#[test]
fn test_into_matrix()
{
    let v = Vec3::create(&[1.0, 2.0, 3.0]);

    let v_expected = Matrix 
        { 
            shape: NewShape(3, 1), 
            elements: HashMap::from([(Position(0, 0), 1.0), (Position(1, 0), 2.0), (Position(2, 0), 3.0)]) 
        };

    assert_eq!(v.into_matrix(), v_expected);
}
