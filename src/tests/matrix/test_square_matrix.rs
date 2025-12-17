#![allow(unused_imports)]
// external imports
use std::collections::HashMap;

use crate::{BasicOperationsTrait, IntoMatrixTrait, Position, Shape};
use crate::{Matrix, SquareMatrix};

#[test]
fn test_create() {
    let m = SquareMatrix::create(2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = SquareMatrix {
        shape: Shape(2, 2),
        elements: HashMap::from([
            (Position(0, 0), 1.0),
            (Position(0, 1), 2.0),
            (Position(1, 0), 3.0),
            (Position(1, 1), 4.0),
        ]),
    };

    assert_eq!(m, expected);
}

#[test]
fn test_get_shape() {
    let m = SquareMatrix::create(2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = &Shape(2, 2);

    assert_eq!(m.get_shape(), expected);
}

#[test]
fn test_get_mut_shape() {
    let mut m = SquareMatrix::create(2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = &mut Shape(2, 2);

    assert_eq!(m.get_mut_shape(), expected);
}

#[test]
fn test_get_elements() {
    let m = SquareMatrix::create(2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = &HashMap::from([
        (Position(0, 0), 1.0),
        (Position(0, 1), 2.0),
        (Position(1, 0), 3.0),
        (Position(1, 1), 4.0),
    ]);

    assert_eq!(m.get_elements(), expected);
}

#[test]
fn test_get_mut_elements() {
    let mut m = SquareMatrix::create(2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = &mut HashMap::from([
        (Position(0, 0), 1.0),
        (Position(0, 1), 2.0),
        (Position(1, 0), 3.0),
        (Position(1, 1), 4.0),
    ]);

    assert_eq!(m.get_mut_elements(), expected);
}

#[test]
fn test_into_matrix() {
    let m = SquareMatrix::create(2, &[1.0, 2.0, 3.0, 4.0]);

    let expected = Matrix {
        shape: Shape(2, 2),
        elements: HashMap::from([
            (Position(0, 0), 1.0),
            (Position(0, 1), 2.0),
            (Position(1, 0), 3.0),
            (Position(1, 1), 4.0),
        ]),
    };

    assert_eq!(m.into_matrix(), expected);
}

#[test]
fn test_create_sparse_empty() {
    let m = SquareMatrix::<f64>::create(4, &[]);
    assert_eq!(m.get_elements().len(), 0);
}

#[test]
fn test_add_value_inserts_and_removes_zero() {
    let mut m = SquareMatrix::create(4, &[]);
    m.add_value(Position(1, 2), 3.0);
    assert_eq!(*m.get_elements().get(&Position(1, 2)).unwrap(), 3.0);

    m.add_value(Position(1, 2), -3.0);
    assert!(m.get_elements().get(&Position(1, 2)).is_none());
}

#[test]
fn test_to_dense_values() {
    let mut m = SquareMatrix::create(3, &[]);
    m.add_value(Position(0, 0), 1.0);
    m.add_value(Position(1, 2), 5.0);

    let dense = m.to_dense_values();
    assert_eq!(dense.len(), 9);
    assert_eq!(dense[0 * 3 + 0], 1.0);
    assert_eq!(dense[1 * 3 + 2], 5.0);
    assert_eq!(dense[2 * 3 + 2], 0.0);
}
