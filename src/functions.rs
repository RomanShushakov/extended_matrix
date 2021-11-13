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


pub fn try_to_compact_matrix<T, V>(ref_symmetric_matrix: &ExtendedMatrix<T, V>)
    -> Result<(Vec<V>, Vec<i64>), String>
    where T: Copy + Debug + From<u8> + Add<Output=T> + Sub<Output=T> + Mul<Output=T> +
             Div<Output=T> + Rem<Output=T> + Eq + Hash + AddAssign + SubAssign + PartialOrd + Ord +
             'static,
          V: Copy + Debug + From<f32> + Into<f64> + Add<Output=V> + Sub<Output=V> + Mul<Output=V> +
             Div<Output=V> + PartialEq + AddAssign + SubAssign + MulAssign + PartialOrd + 'static,
{
    let shape = ref_symmetric_matrix.copy_shape();
    let mut a = Vec::new();
    let mut maxa = Vec::new();
    let mut index = 0i64;
    let mut column = T::from(0u8);
    while column < shape.1
    {
        if ref_symmetric_matrix.copy_element_value_or_zero(
            MatrixElementPosition::create(column, column))? == V::from(0f32)
        {
            let error_message = format!("Try to compact matrix action: Diagonal \
                element {:?}, {:?} equals to zero", column, column);
            return Err(error_message);
        }
        let mut skyline = T::from(0u8);
        'skyline: while skyline < column
        {
            let current_element_value = ref_symmetric_matrix.copy_element_value_or_zero(
                MatrixElementPosition::create(skyline, column))?;
            if current_element_value != V::from(0f32)
            {
                break 'skyline;
            }
            skyline += T::from(1u8);
        }
        let mut row = column;
        maxa.push(index);
        index += 1;
        if row > T::from(0u8)
        {
            while row > skyline
            {
                let current_element_value = ref_symmetric_matrix.copy_element_value_or_zero(
                    MatrixElementPosition::create(row, column))?;
                a.push(current_element_value);
                row -= T::from(1u8);
                if row != column
                {
                    index += 1;
                }
            }
            let current_element_value = ref_symmetric_matrix.copy_element_value_or_zero(
                MatrixElementPosition::create(row, column))?;
            a.push(current_element_value);
        }
        else
        {
            let current_element_value = ref_symmetric_matrix.copy_element_value_or_zero(
                MatrixElementPosition::create(row, column))?;
            a.push(current_element_value);
        }
        column += T::from(1u8);
    }
    
    maxa.push(index);
    Ok((a, maxa))
}



pub fn matrix_element_value_extractor<T, V>(row: T, column: T, ref_matrix: &ExtendedMatrix<T, V>)
    -> Result<V, String>
    where T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> +
             Rem<Output = T> + Copy + Debug + Eq + Hash + SubAssign + PartialOrd + AddAssign +
             From<u8> + Ord + 'static,
          V: Add<Output = V> + Mul<Output = V> + Sub<Output = V> + Div<Output = V> + Copy + Debug +
             PartialEq + AddAssign + MulAssign + SubAssign + Into<f64> + From<f32> + PartialOrd +
             'static
{
    ref_matrix.copy_element_value_or_zero(MatrixElementPosition::create(row, column))
}
