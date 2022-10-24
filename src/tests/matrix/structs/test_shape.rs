#![allow(unused_imports)]

use crate::matrix::NewShape;


#[test]
fn test_update()
{
    let mut s = NewShape(2, 3);

    let expected = NewShape(3, 2); 

    s.update(3, 2);

    assert_eq!(s, expected);
}


#[test]
fn test_swap_rows_and_columns_number()
{
    let mut s = NewShape(1, 3);

    let expected = NewShape(3, 1); 

    s.swap_rows_number_and_columns_number();

    assert_eq!(s, expected);
}