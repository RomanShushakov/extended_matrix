#![allow(unused_imports)]

use crate::{Vector3, VectorTrait, BasicOperationsTrait, Matrix};


#[test]
fn test_norm()
{
    let v = Vector3::create(&[1.0, 2.0, 3.0]);
    assert_eq!(v.norm(), Ok(14f64.sqrt()));
}


#[test]
fn test_dot_product()
{
    let v_1 = Vector3::create(&[1.0, 3.0, -5.0]);
    let v_2 = Vector3::create(&[4.0, -2.0, -1.0]);
    let v_3 = v_1.transpose();
    let v_4 = v_2.transpose();
    assert_eq!(v_1.dot_product(&v_2), Ok(3.0));
    assert_eq!(v_3.dot_product(&v_2), Ok(3.0));
    assert_eq!(v_3.dot_product(&v_4), Ok(3.0));
    assert_eq!(v_1.dot_product(&v_4), Ok(3.0));
}
