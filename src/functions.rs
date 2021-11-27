use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Mul, Add, Sub, Div, Rem, SubAssign, AddAssign, MulAssign};
use std::fmt::Debug;

use finite_element_method::my_float::MyFloatTrait;

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


pub fn conversion_usize_into_uint<T>(u: usize) -> T
    where T: PartialOrd + AddAssign + From<u8>
{
    let mut n = T::from(0u8);
    for _ in 0..u
    {
        n += T::from(1u8);
    }
    n
}


pub fn try_to_compact_matrix<T, V>(ref_symmetric_matrix: &ExtendedMatrix<T, V>)
    -> Result<(Vec<V>, Vec<i64>), String>
    where T: Copy + Debug + From<u8> + Add<Output=T> + Sub<Output=T> + Mul<Output=T> +
             Div<Output=T> + Rem<Output=T> + Eq + Hash + AddAssign + SubAssign + PartialOrd + Ord +
             'static,
          V: Copy + Debug + From<f32> + Into<f64> + Add<Output=V> + Sub<Output=V> + Mul<Output=V> +
             Div<Output=V> + PartialEq + AddAssign + SubAssign + MulAssign + PartialOrd + MyFloatTrait + 
             'static,
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
             MyFloatTrait + 'static
{
    ref_matrix.copy_element_value_or_zero(MatrixElementPosition::create(row, column))
}


pub fn pivot<T, V>(a: &ExtendedMatrix<T, V>, o: &mut [usize], s: &[V], n: usize, k: usize)
    -> Result<(), String>
    where T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> +
             Rem<Output = T> + Copy + Debug + Eq + Hash + SubAssign + PartialOrd + AddAssign +
             From<u8> + Ord + 'static,
          V: Add<Output = V> + Mul<Output = V> + Sub<Output = V> + Div<Output = V> + Copy + Debug +
             PartialEq + AddAssign + MulAssign + SubAssign + Into<f64> + From<f32> + PartialOrd +
             MyFloatTrait + 'static
{
    let mut p = k;
    let mut big = (matrix_element_value_extractor(conversion_usize_into_uint(o[k]), 
        conversion_usize_into_uint(k), a)? / s[o[k]]).my_abs();
    for ii in (k + 1)..n
    {
        let dummy = (matrix_element_value_extractor(conversion_usize_into_uint(o[ii]), 
            conversion_usize_into_uint(k), a)? / s[o[ii]]).my_abs();
        if dummy > big
        {
            big = dummy;
            p = ii;
        }
    }
    let dummy = o[p];
    o[p] = o[k];
    o[k] = dummy;

    Ok(())
}


pub fn decompose<T, V>(a: &mut ExtendedMatrix<T, V>, n: usize, tol: V, o: &mut [usize], s: &mut [V]) -> Result<(), String>
    where T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> +
             Rem<Output = T> + Copy + Debug + Eq + Hash + SubAssign + PartialOrd + AddAssign +
             From<u8> + Ord + 'static,
          V: Add<Output = V> + Mul<Output = V> + Sub<Output = V> + Div<Output = V> + Copy + Debug +
             PartialEq + AddAssign + MulAssign + SubAssign + Into<f64> + From<f32> + PartialOrd +
             MyFloatTrait + 'static
{
    for i in 0..n
    {
        o[i] = i;
        s[i] = matrix_element_value_extractor(conversion_usize_into_uint(i), T::from(0u8), 
            a)?.my_abs();
        for j in 1..n
        {
            if matrix_element_value_extractor(conversion_usize_into_uint(i), conversion_usize_into_uint(j), 
                a)?.my_abs() > s[i]
            {
                s[i] = matrix_element_value_extractor(conversion_usize_into_uint(i), conversion_usize_into_uint(j), 
                    a)?.my_abs()
            } 
        }
    }

    for k in 0..(n - 1)
    {
        pivot(a, o, s, n, k)?;
        if (matrix_element_value_extractor(conversion_usize_into_uint(o[k]), conversion_usize_into_uint(k), 
            a)? / s[o[k]]).my_abs() < tol
        {
            let error_message = format!("{:?}", matrix_element_value_extractor(conversion_usize_into_uint(o[k]), 
                conversion_usize_into_uint(k), a)? / s[o[k]]);
            return Err(error_message);
        }

        for i in (k + 1)..n
        {
            let factor = matrix_element_value_extractor(
                conversion_usize_into_uint(o[i]), conversion_usize_into_uint(k), 
                a)? / 
                matrix_element_value_extractor(conversion_usize_into_uint(o[k]), conversion_usize_into_uint(k), 
                    a)?;

            let matrix_element_position = MatrixElementPosition::create(conversion_usize_into_uint(o[i]), 
                conversion_usize_into_uint(k));
            a.insert_matrix_element(matrix_element_position, factor, tol);
            
            for j in (k + 1)..n
            {
                let previous_value = matrix_element_value_extractor(conversion_usize_into_uint(o[i]), 
                    conversion_usize_into_uint(j), a)?;
                let neighbour_value = matrix_element_value_extractor(conversion_usize_into_uint(o[k]), 
                    conversion_usize_into_uint(j), a)?;
                let matrix_element_position = MatrixElementPosition::create(
                    conversion_usize_into_uint(o[i]), conversion_usize_into_uint(j));
                a.insert_matrix_element(matrix_element_position, 
                    previous_value - factor * neighbour_value, tol);
            }
        }
    }

    if (matrix_element_value_extractor(conversion_usize_into_uint(o[n - 1]), 
        conversion_usize_into_uint(n - 1), a)? / s[o[n - 1]]).my_abs() < tol
    {
        let error_message = format!("{:?}", matrix_element_value_extractor(conversion_usize_into_uint(o[n - 1]), 
            conversion_usize_into_uint(n - 1), a)? / s[o[n - 1]]);
        return Err(error_message);
    }
    Ok(())
}


pub fn substitute<T, V>(a: &ExtendedMatrix<T, V>, o: &[usize], n: usize, b: &mut [V], x: &mut [V]) -> Result<(), String>
    where T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> +
             Rem<Output = T> + Copy + Debug + Eq + Hash + SubAssign + PartialOrd + AddAssign +
             From<u8> + Ord + 'static,
          V: Add<Output = V> + Mul<Output = V> + Sub<Output = V> + Div<Output = V> + Copy + Debug +
             PartialEq + AddAssign + MulAssign + SubAssign + Into<f64> + From<f32> + PartialOrd +
             MyFloatTrait + 'static
{
    for i in 1..n
    {
        let mut sum = b[o[i]];
        for j in 0..i
        {
            sum = sum - matrix_element_value_extractor(conversion_usize_into_uint(o[i]), 
                conversion_usize_into_uint(j), a)? * b[o[j]];
        }
        b[o[i]] = sum;
    }

    x[n - 1] =  b[o[n - 1]] / matrix_element_value_extractor(conversion_usize_into_uint(o[n - 1]), 
        conversion_usize_into_uint(n - 1), a)?;

    for i in (0..(n - 1)).rev()
    {
        let mut sum = V::from(0f32);
        for j in (i + 1)..n
        {
            sum = sum + matrix_element_value_extractor(conversion_usize_into_uint(o[i]), 
                conversion_usize_into_uint(j), a)? * x[j];
        }
        x[i] = (b[o[i]] - sum) / matrix_element_value_extractor(conversion_usize_into_uint(o[i]), 
            conversion_usize_into_uint(i), a)?;
    }

    Ok(())
}
