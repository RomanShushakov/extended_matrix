#![allow(unused_imports)]

use std::collections::HashMap;

use crate::{BasicOperationsTrait, IntoMatrixTrait, Position, Shape, VectorTrait};
use crate::{Matrix, SquareMatrix, Vector3};

#[test]
fn test_create() {
    let v = Vector3::create(&[1.0, 2.0, 3.0]);

    let v_expected = Vector3 {
        shape: Shape(3, 1),
        elements: HashMap::from([
            (Position(0, 0), 1.0),
            (Position(1, 0), 2.0),
            (Position(2, 0), 3.0),
        ]),
    };

    assert_eq!(v, v_expected);
}

#[test]
fn test_get_shape() {
    let v = Vector3::create(&[1.0, 2.0, 3.0]);

    let s_expected = &Shape(3, 1);

    assert_eq!(v.get_shape(), s_expected);
}

#[test]
fn test_get_mut_shape() {
    let mut v = Vector3::create(&[1.0, 2.0, 3.0]);

    let s_expected = &mut Shape(3, 1);

    assert_eq!(v.get_mut_shape(), s_expected);
}

#[test]
fn test_get_elements() {
    let v = Vector3::create(&[1.0, 2.0, 3.0]);

    let e_expected = &HashMap::from([
        (Position(0, 0), 1.0),
        (Position(1, 0), 2.0),
        (Position(2, 0), 3.0),
    ]);

    assert_eq!(v.get_elements(), e_expected);
}

#[test]
fn test_get_mut_elements() {
    let mut v = Vector3::create(&[1.0, 2.0, 3.0]);

    let e_expected = &mut HashMap::from([
        (Position(0, 0), 1.0),
        (Position(1, 0), 2.0),
        (Position(2, 0), 3.0),
    ]);

    assert_eq!(v.get_mut_elements(), e_expected);
}

#[test]
fn test_into_matrix() {
    let v = Vector3::create(&[1.0, 2.0, 3.0]);

    let m_expected = Matrix {
        shape: Shape(3, 1),
        elements: HashMap::from([
            (Position(0, 0), 1.0),
            (Position(1, 0), 2.0),
            (Position(2, 0), 3.0),
        ]),
    };

    assert_eq!(v.into_matrix(), m_expected);
}

#[test]
fn test_cross_product() {
    let v_1 = Vector3::create(&[3.0, 3.0, 0.0]);
    let v_2 = Vector3::create(&[0.0, 2.0, 2.0]);
    let v_3 = v_1.transpose();
    let v_4 = v_2.transpose();

    let v_expected = Vector3 {
        shape: Shape(3, 1),
        elements: HashMap::from([
            (Position(0, 0), 6.0),
            (Position(1, 0), -6.0),
            (Position(2, 0), 6.0),
        ]),
    };

    assert_eq!(v_1.cross_product(&v_2), v_expected.clone());
    assert_eq!(v_1.cross_product(&v_4), v_expected.clone());
    assert_eq!(v_3.cross_product(&v_2), v_expected.clone());
    assert_eq!(v_3.cross_product(&v_4), v_expected.clone());
}

#[test]
fn test_get_components() {
    let v_1 = Vector3::create(&[3.0, 3.0, 0.0]);
    let v_2 = v_1.transpose();

    let expected = [3.0, 3.0, 0.0];

    assert_eq!(v_1.get_components(), expected.clone());
    assert_eq!(v_2.get_components(), expected);
}

#[test]
fn test_cosine_angle_between_vectors() {
    let v_1 = Vector3::create(&[3.0, 3.0, 0.0]);
    let v_2 = Vector3::create(&[0.0, 2.0, 2.0]);
    let v_3 = v_1.transpose();
    let v_4 = v_2.transpose();

    let expected = 0.5;

    assert_eq!(v_1.cosine_angle_between_vectors(&v_2), expected);
    assert_eq!(v_1.cosine_angle_between_vectors(&v_4), expected);
    assert_eq!(v_2.cosine_angle_between_vectors(&v_3), expected);
}

#[test]
fn test_projection_perpendicular_to_vector() {
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

    let v_expected_1 = Vector3 {
        shape: Shape(3, 1),
        elements: HashMap::from([
            (Position(0, 0), -0.3333333333333333),
            (Position(1, 0), -0.3333333333333333),
            (Position(2, 0), 0.6666666666666666),
        ]),
    };

    let v_expected_2 = Vector3 {
        shape: Shape(3, 1),
        elements: HashMap::from([
            (Position(0, 0), 0.0),
            (Position(1, 0), 1.0),
            (Position(2, 0), 1.0),
        ]),
    };

    let v_expected_3 = Vector3 {
        shape: Shape(3, 1),
        elements: HashMap::from([
            (Position(0, 0), 0.03539823008849559),
            (Position(1, 0), 7.309734513274336),
            (Position(2, 0), 1.4513274336283186),
        ]),
    };

    let v_expected_4 = Vector3 {
        shape: Shape(3, 1),
        elements: HashMap::from([
            (Position(0, 0), 0.0),
            (Position(1, 0), 0.0),
            (Position(2, 0), 0.0),
        ]),
    };

    assert_eq!(v_1.projection_perpendicular_to_vector(&v_2), v_expected_1);
    assert_eq!(v_3.projection_perpendicular_to_vector(&v_4), v_expected_2);
    assert_eq!(v_5.projection_perpendicular_to_vector(&v_6), v_expected_3);
    assert_eq!(v_7.projection_perpendicular_to_vector(&v_8), v_expected_4);
}

#[test]
fn test_rotation_matrix_to_align_with_vector() {
    let abs_tol = 1e-7;
    let rel_tol = 0.0001;
    let v_1 = Vector3::create(&[10.0, 0.0, 1.0]);
    let v_2 = Vector3::create(&[7.0, -5.0, 5.196]);
    let v_3 = Vector3::create(&[-10.0, 0.0, 1.0]);
    let v_4 = Vector3::create(&[10.0, 0.0, 1.0]);
    let v_5 = Vector3::create(&[-10.0, 0.0, -1.0]);
    let v_6 = Vector3::create(&[0.0, 0.0, 5.0]);
    let v_7 = Vector3::create(&[0.0, 0.0, -5.0]);

    let m_expected_1 = Matrix::create(
        3,
        3,
        &[
            0.7459255332342182,
            0.4824210341757528,
            -0.4592004406074151,
            -0.5076857397612721,
            0.8581108060421796,
            0.07681818923074477,
            0.4311035705167576,
            0.17582886662444727,
            0.8850050401829728,
        ],
    );

    let m_expected_2 = Matrix::create(
        3,
        3,
        &[
            -0.9801980198019802,
            -0.0,
            -0.19801980198019803,
            0.0,
            1.0,
            -0.0,
            0.19801980198019803,
            0.0,
            -0.9801980198019802,
        ],
    );

    let m_expected_3 = Matrix::create(3, 3, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);

    let m_expected_4 = Matrix::create(3, 3, &[-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0]);

    let m_expected_5 = Matrix::create(3, 3, &[-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0]);

    let rotation_matrix_1 = v_1.rotation_matrix_to_align_with_vector(&v_2, rel_tol, abs_tol);
    let rotation_matrix_2 = v_1.rotation_matrix_to_align_with_vector(&v_3, rel_tol, abs_tol);
    let rotation_matrix_3 = v_1.rotation_matrix_to_align_with_vector(&v_4, rel_tol, abs_tol);
    let rotation_matrix_4 = v_1.rotation_matrix_to_align_with_vector(&v_5, rel_tol, abs_tol);
    let rotation_matrix_5 = v_6.rotation_matrix_to_align_with_vector(&v_7, rel_tol, abs_tol);

    assert_eq!(rotation_matrix_1, Ok(m_expected_1));
    assert_eq!(rotation_matrix_2, Ok(m_expected_2));
    assert_eq!(rotation_matrix_3, Ok(m_expected_3));
    assert_eq!(rotation_matrix_4, Ok(m_expected_4));
    assert_eq!(rotation_matrix_5, Ok(m_expected_5));
}
