use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Mul, Add, Sub, Div, Rem, SubAssign, AddAssign, MulAssign};
use std::fmt::Debug;

use crate::basic_matrix::basic_matrix::{BasicMatrixType};

use crate::extended_matrix::ExtendedMatrix;
use crate::extended_matrix::Operation;

use crate::shape::Shape;
use crate::matrix_element_position::MatrixElementPosition;


pub fn conversion_uint_into_usize<T>(uint: T) -> usize
    where T: PartialOrd + AddAssign + From<u8>
{
    let mut n = 0usize;
    let mut m = T::from(0u8);
    while m < uint
    {
        n += 1usize;
        m += T::from(1u8);
    }
    n
}
