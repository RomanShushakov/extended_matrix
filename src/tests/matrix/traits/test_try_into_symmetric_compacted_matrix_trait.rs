#![allow(unused_imports)]

use crate::{Position, SquareMatrix, TryIntoSymmetricCompactedMatrixTrait};

#[test]
fn test_try_into_symmetric_compacted_matrix() {
    let sm_1 = SquareMatrix::create(
        4,
        &[
            5.0, -4.0, 1.0, 0.0, -4.000001, 6.0, -4.0, 1.0, 1.0, -4.0, 6.0, -4.0, 0.0, 1.0, -4.0,
            5.0,
        ],
    );
    let sm_2 = SquareMatrix::create(
        5,
        &[
            2.0, -2.0, 0.0, 0.0, -1.0, -2.0, 3.0, -2.0, 0.0, 0.0, 0.0, -2.0, 5.0, -3.0, 0.0, 0.0,
            0.0, -3.0, 10.0, 4.0, -1.0, 0.0, 0.0, 4.0, 10.0,
        ],
    );
    let sm_3 = SquareMatrix::create(
        4,
        &[
            5.0, -4.0, 1.0, 0.0, -4.000001, 6.0, -4.0, 1.0, 1.0, -4.0, 6.0, -4.0, 0.0, 1.0, -4.0,
            0.0,
        ],
    );
    let sm_4 = SquareMatrix::create(
        4,
        &[
            5.0, -4.0, 1.0, 0.0, -4.25, 6.0, -4.0, 1.0, 1.0, -4.0, 6.0, -4.0, 0.0, 1.0, -4.0, 0.0,
        ],
    );

    let expected_a_1 = vec![5.0, 6.0, -4.0, 6.0, -4.0, 1.0, 5.0, -4.0, 1.0];
    let expected_maxa_1 = vec![0i64, 1, 3, 6, 9];
    let expected_a_2 = vec![
        2.0, 3.0, -2.0, 5.0, -2.0, 10.0, -3.0, 10.0, 4.0, 0.0, 0.0, -1.0,
    ];
    let expected_maxa_2 = vec![0, 1, 3, 5, 7, 12];

    assert_eq!(
        sm_1.try_into_symmetric_compacted_matrix(1e-6),
        Ok((expected_a_1, expected_maxa_1))
    );
    assert_eq!(
        sm_2.try_into_symmetric_compacted_matrix(1e-6),
        Ok((expected_a_2, expected_maxa_2))
    );
    assert_eq!(
        sm_3.try_into_symmetric_compacted_matrix(1e-6),
        Err("Diagonal element [3, 3] equals to zero!".to_string())
    );
    assert_eq!(
        sm_4.try_into_symmetric_compacted_matrix(1e-6),
        Err("Element [0, 1] does not match with [1, 0]!".to_string())
    );
}

#[test]
fn test_forced_into_symmetric_compacted_matrix() {
    let mut warnings_1 = Vec::new();
    let sm_1 = SquareMatrix::create(
        4,
        &[
            5.0, -4.0, 1.0, 0.0, -4.000001, 6.0, -4.0, 1.0, 1.0, -4.0, 6.0, -4.0, 0.0, 1.0, -4.0,
            5.0,
        ],
    );
    let mut warnings_2 = Vec::new();
    let sm_2 = SquareMatrix::create(
        5,
        &[
            2.0, -2.0, 0.0, 0.0, -1.0, -2.0, 3.0, -2.0, 0.0, 0.0, 0.0, -2.0, 5.0, -3.0, 0.0, 0.0,
            0.0, -3.0, 10.0, 4.0, -1.0, 0.0, 0.0, 4.0, 10.0,
        ],
    );
    let mut warnings_3 = Vec::new();
    let sm_3 = SquareMatrix::create(
        4,
        &[
            5.0, -4.0, 1.0, 0.0, -4.25, 6.0, -4.0, 1.0, 1.0, -4.0, 6.0, -4.0, 0.0, 1.0, -4.0, 0.0,
        ],
    );

    let expected_warnings_1: Vec<Vec<(Position, f64)>> = Vec::new();
    let expected_a_1 = vec![5.0, 6.0, -4.0, 6.0, -4.0, 1.0, 5.0, -4.0, 1.0];
    let expected_maxa_1 = vec![0i64, 1, 3, 6, 9];
    let expected_warnings_2: Vec<Vec<(Position, f64)>> = Vec::new();
    let expected_a_2 = vec![
        2.0, 3.0, -2.0, 5.0, -2.0, 10.0, -3.0, 10.0, 4.0, 0.0, 0.0, -1.0,
    ];
    let expected_maxa_2 = vec![0, 1, 3, 5, 7, 12];
    let expected_warnings_3: Vec<Vec<(Position, f64)>> = vec![
        vec![(Position(0, 1), -4.0), (Position(1, 0), -4.25)],
        vec![(Position(3, 3), 0.0)],
    ];
    let expected_a_3 = vec![5.0, 6.0, -4.0, 6.0, -4.0, 1.0, 0.0, -4.0, 1.0];
    let expected_maxa_3 = vec![0i64, 1, 3, 6, 9];

    assert_eq!(
        sm_1.forced_into_symmetric_compacted_matrix(1e-6, &mut warnings_1),
        (expected_a_1, expected_maxa_1)
    );
    assert_eq!(warnings_1, expected_warnings_1);
    assert_eq!(
        sm_2.forced_into_symmetric_compacted_matrix(1e-6, &mut warnings_2),
        (expected_a_2, expected_maxa_2)
    );
    assert_eq!(warnings_2, expected_warnings_2);
    assert_eq!(
        sm_3.forced_into_symmetric_compacted_matrix(1e-6, &mut warnings_3),
        (expected_a_3, expected_maxa_3)
    );
    assert_eq!(warnings_3, expected_warnings_3);
}
