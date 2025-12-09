#![allow(unused_imports)]
// external imports
use std::collections::HashMap;

use crate::{BasicOperationsTrait, IntoMatrixTrait, Position, Shape};
use crate::{Matrix, Vector};

#[test]
fn test_create() {
    let v = Vector::create(&[1.0, 2.0, 3.0, 4.0]);

    let v_expected = Vector {
        shape: Shape(4, 1),
        elements: HashMap::from([
            (Position(0, 0), 1.0),
            (Position(1, 0), 2.0),
            (Position(2, 0), 3.0),
            (Position(3, 0), 4.0),
        ]),
    };

    assert_eq!(v, v_expected);
}

#[test]
fn test_get_shape() {
    let v = Vector::create(&[1.0, 2.0, 3.0, 4.0]);

    let s_expected = &Shape(4, 1);

    assert_eq!(v.get_shape(), s_expected);
}

#[test]
fn test_get_mut_shape() {
    let mut v = Vector::create(&[1.0, 2.0, 3.0, 4.0]);

    let s_expected = &mut Shape(4, 1);

    assert_eq!(v.get_mut_shape(), s_expected);
}

#[test]
fn test_get_elements() {
    let v = Vector::create(&[1.0, 2.0, 3.0, 4.0]);

    let e_expected = &HashMap::from([
        (Position(0, 0), 1.0),
        (Position(1, 0), 2.0),
        (Position(2, 0), 3.0),
        (Position(3, 0), 4.0),
    ]);

    assert_eq!(v.get_elements(), e_expected);
}

#[test]
fn test_get_mut_elements() {
    let mut v = Vector::create(&[1.0, 2.0, 3.0, 4.0]);

    let e_expected = &mut HashMap::from([
        (Position(0, 0), 1.0),
        (Position(1, 0), 2.0),
        (Position(2, 0), 3.0),
        (Position(3, 0), 4.0),
    ]);

    assert_eq!(v.get_mut_elements(), e_expected);
}

#[test]
fn test_into_matrix() {
    let v = Vector::create(&[1.0, 2.0, 3.0, 4.0]);

    let m_expected = Matrix {
        shape: Shape(4, 1),
        elements: HashMap::from([
            (Position(0, 0), 1.0),
            (Position(1, 0), 2.0),
            (Position(2, 0), 3.0),
            (Position(3, 0), 4.0),
        ]),
    };

    assert_eq!(v.into_matrix(), m_expected);
}
