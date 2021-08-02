use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Mul, Add, Sub, Div, Rem, SubAssign, AddAssign, MulAssign};
use std::fmt::Debug;

use crate::one::One;

use crate::basic_matrix::basic_matrix::{MatrixElementPosition, Shape};

use crate::extended_matrix::ExtendedMatrix;
use crate::extended_matrix::Operation;


pub(super) fn matrices_dimensions_conformity_check<'a, T, V>(lhs: &'a ExtendedMatrix<T, V>,
    rhs: &'a ExtendedMatrix<T, V>, operation: Operation) -> Result<(T, Shape<T>), &'a str>
    where T: Copy +PartialEq + Mul<Output = T> + Add<Output = T> + Sub<Output = T> +
             Div<Output = T> + Rem<Output = T> + Default + One + AddAssign + Eq + Hash +
             SubAssign + Debug + PartialOrd + 'static,
          V: Copy + Default + Mul<Output = V> + Div<Output = V> + Sub<Output = V> +
             Add<Output = V> + Debug + PartialEq + AddAssign + MulAssign + Into<f64> + One +
             SubAssign + 'static,
{
    let lhs_shape = lhs.get_shape();
    let rhs_shape = rhs.get_shape();
    match operation
    {
        Operation::Multiplication =>
            {
                if lhs_shape.1 != rhs_shape.0
                {
                    return Err("Extended matrix: Shapes of matrices does not conform to each other!");
                }
                Ok((lhs_shape.1, Shape(lhs_shape.0, rhs_shape.1)))
            },
        Operation::Addition =>
            {
                if lhs_shape.0 != rhs_shape.0 || lhs_shape.1 != rhs_shape.1
                {
                    return Err("Extended matrix: Shapes of matrices does not conform to each other!");
                }
                Ok((lhs_shape.1, Shape(lhs_shape.0, rhs_shape.1)))
            }
        Operation::Subtraction =>
            {
                if lhs_shape.0 != rhs_shape.0 || lhs_shape.1 != rhs_shape.1
                {
                    return Err("Extended matrix: Shapes of matrices does not conform to each other!");
                }
                Ok((lhs_shape.1, Shape(lhs_shape.0, rhs_shape.1)))
            }
    }

}


pub fn extract_element_value<T, V>(row: T, column: T,
    elements_values: &HashMap<MatrixElementPosition<T>, V>) -> V
    where T: Hash + Eq + Copy,
          V: Copy + Default,
{
    let element_position = MatrixElementPosition::create(row, column);
    let element_value =
        if let Some(value) = elements_values.get(&element_position)
        {
            *value
        }
        else { V::default() };
    element_value
}


pub(super) fn remove_zero_values<T, V>(indexes: &mut Vec<T>, values: &mut Vec<V>, tolerance: V)
    where V: Copy + Default + PartialEq + Into<f64>
{
    let mut i = indexes.len() - 1;
    while i > 0
    {
        if values[i].into().abs() < tolerance.into()
        {
            indexes.remove(i);
            values.remove(i);
        }
        i -= 1;
    }
}


pub fn conversion_uint_into_usize<T>(uint: T) -> usize
    where T: Default + PartialOrd + One + AddAssign
{
    let mut n = 0usize;
    let mut m = T::default();
    while m < uint
    {
        n += 1usize;
        m += T::one();
    }
    n
}
