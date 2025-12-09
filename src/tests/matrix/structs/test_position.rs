#![allow(unused_imports)]

use crate::Position;

#[test]
fn test_swap_row_and_column() {
    let mut p = Position(0, 3);

    let expected = Position(3, 0);

    p.swap_row_and_column();

    assert_eq!(p, expected);
}
