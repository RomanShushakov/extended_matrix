#![allow(unused_imports)]

use crate::{Vector3, VectorTrait, Matrix};


#[test]
fn test_norm()
{
    let v = Vector3::create(&[1.0, 2.0, 3.0]);
    assert_eq!(v.norm(), 14f64.sqrt());
}


#[test]
fn test_dot_product()
{
    let v_1 = Vector3::create(&[1.0, 3.0, -5.0]);
    let v_2 = Vector3::create(&[4.0, -2.0, -1.0]);
    let m = Matrix::create(3, 1,vec![1.0, 3.0, -5.0]);
    assert_eq!(v_1.dot_product(&v_2), Ok(3.0));
    assert_eq!(v_2.dot_product(&m), Ok(3.0));
}
