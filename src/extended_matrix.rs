use std::fmt::Debug;
use std::convert::{From, Into};
use std::ops::{Mul, Add, Sub, Div, Rem, MulAssign, AddAssign, SubAssign};
use std::hash::Hash;
use std::collections::HashMap;

use crate::basic_matrix::basic_matrix::{BasicMatrixTrait, BasicMatrixType};
use crate::basic_matrix::non_symmetric_matrix::NonSymmetricMatrix;

use crate::shape::Shape;
use crate::matrix_element_position::MatrixElementPosition;

use crate::new_extended_matrix::Operation;

use crate::functions::
{
    matrices_dimensions_conformity_check, copy_element_value_or_zero, remove_zero_values,
    conversion_uint_into_usize
};


#[derive(Clone)]
pub struct ExtendedMatrix<T, V>
{
    tolerance: V,
    basic_matrix: Box<dyn BasicMatrixTrait<T, V>>
}


impl<T, V> ExtendedMatrix<T, V>
    where T: Copy + Debug + Mul<Output = T> + PartialOrd + Add<Output = T> + Sub<Output = T> +
             Div<Output = T> + Rem<Output = T> + Eq + Hash + SubAssign + AddAssign + From<u8> +
             'static,
          V: Copy + Debug + PartialEq + AddAssign + MulAssign + Mul<Output = V> + Div<Output = V> +
             SubAssign + Sub<Output = V> + Add<Output = V> + Into<f64> + From<f32> + 'static,
{
    pub fn show_matrix<F>(&self, f: F)
        where F: Fn(&str)
    {
        let shape = self.basic_matrix.copy_shape();

        let mut row = T::from(0u8);
        while row < shape.0
        {
            let mut row_str = String::new();
            let mut column = T::from(0u8);
            while column < shape.1
            {
                row_str += &format!("{:?}, ",
                    self.basic_matrix.read_element_value(row, column).unwrap());
                column += T::from(1u8);
            }
            f(&format!("{}", row_str));
            row += T::from(1u8);
        }
    }


    pub fn create(rows_number: T, columns_number: T, all_elements_values: Vec<V>, tolerance: V)
        -> Self
    {
        let mut elements_indexes = Vec::new();
        let mut elements_values = Vec::new();

        let mut index = T::from(0u8);
        for value in all_elements_values.into_iter()
        {
            if value.into().abs() > tolerance.into()
            {
                elements_indexes.push(index);
                elements_values.push(value);
            }
            index += T::from(1u8);
        }

        let basic_matrix = Box::new(NonSymmetricMatrix::create(
            rows_number, columns_number, elements_indexes,
            elements_values));

        let basic_matrix = basic_matrix.into_symmetric();
        ExtendedMatrix { tolerance, basic_matrix }
    }


    pub fn copy_shape(&self) -> Shape<T>
    {
        self.basic_matrix.copy_shape()
    }


    pub fn copy_all_elements_values(&self) -> HashMap<MatrixElementPosition<T>, V>
    {
        self.basic_matrix.copy_all_elements_values()
    }


    pub fn transpose(&mut self)
    {
        self.basic_matrix.transpose();
    }


    pub fn add_subtract_matrix<'a>(&'a self, other: &'a Self, operation: Operation)
        -> Result<Self, &'a str>
    {
        let (_, shape) =
            matrices_dimensions_conformity_check(&self, &other, operation)?;
        let lhs_all_elements_values =
            self.basic_matrix.copy_all_elements_values();
        let rhs_all_elements_values =
            other.basic_matrix.copy_all_elements_values();
        let mut elements_indexes = Vec::new();
        let mut elements_values = Vec::new();

        let mut index = T::from(0u8);
        while index < shape.0 * shape.1
        {
            let current_lhs_element_value = copy_element_value_or_zero(
                    index / shape.1, index % shape.1,
                    &lhs_all_elements_values
                );
            let current_rhs_element_value = copy_element_value_or_zero(
                    index / shape.1, index % shape.1,
                    &rhs_all_elements_values
                );
            let value =
                {
                    match operation
                    {
                        Operation::Addition =>
                            current_lhs_element_value + current_rhs_element_value,
                        Operation::Subtraction =>
                            current_lhs_element_value - current_rhs_element_value,
                        Operation::Multiplication =>
                            return Err("Extended matrix: Multiplication operation could not be \
                                applied for add_subtract function!"),
                    }
                };
            if value.into().abs() > self.tolerance.into()
            {
                elements_indexes.push(index);
                elements_values.push(value);
            }
            index += T::from(1u8);
        }

        let basic_matrix = Box::new(NonSymmetricMatrix::create(
            shape.0, shape.1, elements_indexes, elements_values));

        let basic_matrix = basic_matrix.into_symmetric();
        Ok(ExtendedMatrix { tolerance: self.tolerance, basic_matrix })
    }


    pub fn add_matrix<'a>(&'a self, other: &'a Self) -> Result<Self, &'a str>
    {
        self.add_subtract_matrix(other, Operation::Addition)
    }


    pub fn subtract_matrix<'a>(&'a self, other: &'a Self) -> Result<Self, &'a str>
    {
        self.add_subtract_matrix(other, Operation::Subtraction)
    }


    pub fn add_sub_matrix(&mut self, other: &Self, self_positions: &[MatrixElementPosition<T>],
        other_positions: &[MatrixElementPosition<T>], tolerance: V)
    {
        let lhs_shape = self.basic_matrix.copy_shape();
        let lhs_all_elements_values =
            self.basic_matrix.copy_all_elements_values();
        let rhs_all_elements_values =
            other.basic_matrix.copy_all_elements_values();
        let mut elements_indexes = Vec::new();
        let mut elements_values = Vec::new();
        for (lhs_position, rhs_position) in
            self_positions.iter().zip(other_positions)
        {
            let current_lhs_element_value = copy_element_value_or_zero(
                *lhs_position.ref_row(), *lhs_position.ref_column(),
                &lhs_all_elements_values
                );
            let current_rhs_element_value = copy_element_value_or_zero(
                *rhs_position.ref_row(), *rhs_position.ref_column(),
                &rhs_all_elements_values
                );
            let value = current_lhs_element_value + current_rhs_element_value;
            if value.into().abs() > tolerance.into()
            {
                elements_indexes.push(*lhs_position.ref_row() * lhs_shape.1 + *lhs_position.ref_column());
                elements_values.push(value);
            }
        }

        let mut index = T::from(0u8);
        while index < lhs_shape.0 * lhs_shape.1
        {
            if let None = self_positions.iter().position(|existed_matrix_element_position|
                {
                    let matrix_element_position =
                        MatrixElementPosition::create(index / lhs_shape.1,
                        index % lhs_shape.1);
                    *existed_matrix_element_position == matrix_element_position
                })
            {
                let value = copy_element_value_or_zero(
                    index / lhs_shape.1, index % lhs_shape.1,
                    &lhs_all_elements_values
                );
                if value.into().abs() > tolerance.into()
                {
                    elements_indexes.push(index);
                    elements_values.push(value);
                }
            }
            index += T::from(1u8);
        }

        let basic_matrix = Box::new(NonSymmetricMatrix::create(
            lhs_shape.0, lhs_shape.1, elements_indexes, elements_values));

        let basic_matrix = basic_matrix.into_symmetric();
        self.basic_matrix = basic_matrix;
    }


    pub fn multiply_by_number(&mut self, number: V)
    {
        self.basic_matrix.multiply_by_number(number);
    }


    pub fn multiply_by_matrix<'a>(&'a self, other: &'a Self)
        -> Result<Self, &'a str>
    {
        let (basic_dimension, shape) =
            matrices_dimensions_conformity_check(&self, &other,
        Operation::Multiplication)?;
        let lhs_all_elements_values =
            self.basic_matrix.copy_all_elements_values();
        let rhs_all_elements_values =
            other.basic_matrix.copy_all_elements_values();
        let mut elements_indexes = Vec::new();
        let mut elements_values = Vec::new();

        let mut index = T::from(0u8);
        while index < shape.0 * shape.1
        {
            let mut value = V::from(0f32);

            let mut k = T::from(0u8);
            while k < basic_dimension
            {
                let current_lhs_element_value = copy_element_value_or_zero(
                        index / shape.1, k,
                        &lhs_all_elements_values
                    );
                let current_rhs_element_value = copy_element_value_or_zero(
                        k, index % shape.1,
                        &rhs_all_elements_values
                    );
                value += current_lhs_element_value * current_rhs_element_value;
                k += T::from(1u8);
            }

            if value.into().abs() > self.tolerance.into()
            {
                elements_indexes.push(index);
                elements_values.push(value);
            }
            index += T::from(1u8);
        }

        let basic_matrix = Box::new(NonSymmetricMatrix::create(
            shape.0, shape.1, elements_indexes, elements_values));

        let basic_matrix = basic_matrix.into_symmetric();
        Ok(ExtendedMatrix { tolerance: self.tolerance, basic_matrix })
    }


    pub fn naive_gauss_elimination<'a>(&'a self, other: &'a Self)
        -> Result<Self, &'a str>
    {
        let (basic_dimension, shape) =
            matrices_dimensions_conformity_check(&self, &other,
             Operation::Multiplication)?;
        let mut lhs_all_elements_values =
            self.basic_matrix.copy_all_elements_values();
        let mut rhs_all_elements_values =
            other.basic_matrix.copy_all_elements_values();

        let mut elements_values = Vec::new();
        let mut count = T::from(0u8);
        while count < shape.0
        {
            elements_values.push(V::from(0f32));
            count += T::from(1u8);
        }

        let mut k = T::from(0u8);
        while k < basic_dimension - T::from(1u8)
        {
            let mut i = k + T::from(1u8);
            while i < basic_dimension
            {
                let current_lhs_element_value = copy_element_value_or_zero(i, k,
                                                                           &lhs_all_elements_values);

                let current_diag_lhs_element_value = copy_element_value_or_zero(k, k,
                                                                                &lhs_all_elements_values);

                if current_diag_lhs_element_value == V::from(0f32)
                {
                    return Err("Extended matrix: Matrix is singular!");
                }

                let current_coefficient =
                    current_lhs_element_value / current_diag_lhs_element_value;

                let mut j = k + T::from(1u8);
                while j < basic_dimension
                {
                    let current_lhs_element_value = copy_element_value_or_zero(k, j,
                                                                               &lhs_all_elements_values);

                    *lhs_all_elements_values
                        .entry(MatrixElementPosition::create(i, j))
                        .or_insert(V::from(0f32)) -=
                            current_coefficient * current_lhs_element_value;
                    j += T::from(1u8);
                }

                let current_rhs_element_value = copy_element_value_or_zero(k,
                                                                           T::from(0u8), &rhs_all_elements_values);
                *rhs_all_elements_values
                    .entry(MatrixElementPosition::create(i, T::from(0u8)))
                    .or_insert(V::from(0f32)) -=
                        current_coefficient * current_rhs_element_value;
                i += T::from(1u8);
            }
            k += T::from(1u8);
        }

        let rhs_element_value = copy_element_value_or_zero(basic_dimension - T::from(1u8),
                                                           T::from(0u8), &rhs_all_elements_values);

        let lhs_element_value = copy_element_value_or_zero(basic_dimension - T::from(1u8),
                                                           basic_dimension - T::from(1u8), &lhs_all_elements_values);

        let n = conversion_uint_into_usize(basic_dimension - T::from(1u8));

        elements_values[n] = rhs_element_value / lhs_element_value;

        let mut i = basic_dimension - T::from(1u8);
        while i > T::from(0u8)
        {
            i -= T::from(1u8);
            let rhs_element_value = copy_element_value_or_zero(i, T::from(0u8),
                                                               &rhs_all_elements_values);

            let mut sum = rhs_element_value;

            let mut j = i + T::from(1u8);
            while j < basic_dimension
            {
                let lhs_element_value = copy_element_value_or_zero(i, j,
                                                                   &lhs_all_elements_values);

                let n = conversion_uint_into_usize(j);

                sum -= lhs_element_value * elements_values[n];
                j += T::from(1u8);
            }

            let lhs_element_value = copy_element_value_or_zero(i, i,
                                                               &lhs_all_elements_values);

            let n = conversion_uint_into_usize(i);

            elements_values[n] = sum / lhs_element_value;
        }

        Ok(ExtendedMatrix::create(shape.0, shape.1,
            elements_values, self.tolerance))
    }


    pub fn lu_decomposition(&self) -> Result<(Self, Self), &str>
    {
        let shape = self.basic_matrix.copy_shape();
        if (shape.0 != shape.1) || shape.0 < T::from(2u8)
        {
            return Err("Extended matrix: Matrix could not be decomposed!");
        }
        let mut l_elements_indexes = Vec::new();
        let mut l_elements_values= Vec::new();

        let mut i = T::from(0u8);
        while i < shape.0
        {
            l_elements_indexes.push(i * shape.1 + i);
            l_elements_values.push(V::from(1f32));
            i += T::from(1u8);
        }

        let mut all_elements_values =
            self.basic_matrix.copy_all_elements_values();
        let mut u_elements_indexes = Vec::new();
        let mut u_elements_values= Vec::new();

        let mut k = T::from(0u8);
        while k < shape.1
        {
            let current_element_value = copy_element_value_or_zero(T::from(0u8), k,
                                                                   &all_elements_values);
            u_elements_indexes.push(k);
            u_elements_values.push(current_element_value);
            k += T::from(1u8);
        }

        let mut row_number = T::from(0u8);

        while row_number < shape.0 - T::from(1u8)
        {
            let mut i = row_number + T::from(1u8);
            while i < shape.0
            {
                let current_coefficient = copy_element_value_or_zero(i, row_number,
                                                                     &all_elements_values) / copy_element_value_or_zero(row_number,
                                                                                                                        row_number, &all_elements_values);

                l_elements_indexes.push(i * shape.1 + row_number);
                l_elements_values.push(current_coefficient);

                let mut j = T::from(0u8);
                while j < shape.1
                {
                    let current_element_value = copy_element_value_or_zero(i, j,
                                                                           &all_elements_values) - copy_element_value_or_zero(row_number, j,
                                                                                                                              &all_elements_values) * current_coefficient;

                    if let Some(position) = u_elements_indexes.iter().position(|index|
                        *index ==  i * shape.1 + j)
                    {
                        u_elements_values[position] = current_element_value;
                    }
                    else
                    {
                        u_elements_indexes.push(i * shape.1 + j);
                        u_elements_values.push(current_element_value);
                    }
                    *all_elements_values
                        .entry(MatrixElementPosition::create(i, j))
                        .or_insert(V::from(0f32)) = current_element_value;
                    j += T::from(1u8);
                }
                i += T::from(1u8);
            }
            row_number += T::from(1u8);
        }


        remove_zero_values(&mut l_elements_indexes, &mut l_elements_values, self.tolerance);
        let l_basic_matrix = Box::new(NonSymmetricMatrix::create(
            shape.0, shape.1, l_elements_indexes, l_elements_values));

        let l_matrix = ExtendedMatrix { tolerance: self.tolerance, basic_matrix: l_basic_matrix };
        remove_zero_values(&mut u_elements_indexes, &mut u_elements_values, self.tolerance);
        let u_basic_matrix = Box::new(NonSymmetricMatrix::create(
            shape.0, shape.1, u_elements_indexes, u_elements_values));

        let u_matrix = ExtendedMatrix { tolerance: self.tolerance, basic_matrix: u_basic_matrix };
        Ok((l_matrix, u_matrix))
    }


    pub fn determinant(&self) -> Result<V, &str>
    {
        let (_, u_matrix) = self.lu_decomposition()?;
        let u_matrix_elements_values = u_matrix.basic_matrix
            .copy_all_elements_values();
        let shape = u_matrix.basic_matrix.copy_shape();
        let mut determinant = V::from(1f32);

        let mut i = T::from(0u8);
        while i < shape.0
        {
            let current_diag_element_value = copy_element_value_or_zero(i, i,
                                                                        &u_matrix_elements_values);
            determinant *= current_diag_element_value;
            i += T::from(1u8);
        }

        Ok(determinant)
    }


    pub fn inverse(&self) -> Result<Self, &str>
    {
        let (l_matrix, u_matrix) =
            self.lu_decomposition()?;

        let shape = self.basic_matrix.copy_shape();
        let mut inverse_matrix_indexes = Vec::new();
        let mut inverse_matrix_values = Vec::new();

        let mut k = T::from(0u8);
        while k < shape.1
        {
            let unit_column_indexes = vec![k];
            let unit_column_values = vec![V::from(1f32)];

            let basic_unit_column = Box::new(
                NonSymmetricMatrix::create(shape.1, T::from(1u8),
                unit_column_indexes, unit_column_values));

            let unit_column =
                ExtendedMatrix { tolerance: self.tolerance, basic_matrix: basic_unit_column };

            let interim_inverse_column = l_matrix
                .naive_gauss_elimination(&unit_column).unwrap();

            let inverse_column = u_matrix
                .naive_gauss_elimination(&interim_inverse_column).unwrap();

            let all_inverse_column_values =
                inverse_column.basic_matrix.copy_all_elements_values();

            let mut i = T::from(0u8);
            while i < shape.0
            {
                let current_inverse_column_element_value = copy_element_value_or_zero(i,
                                                                                      T::from(0u8), &all_inverse_column_values);

                if current_inverse_column_element_value != V::from(0f32)
                {
                    inverse_matrix_indexes.push(i * shape.1 + k);
                    inverse_matrix_values.push(current_inverse_column_element_value);
                }
                i += T::from(1u8);
            }
            k += T::from(1u8);
        }

        let basic_inverse_matrix = Box::new(
            NonSymmetricMatrix::create(shape.0, shape.1,
                inverse_matrix_indexes, inverse_matrix_values));

        let basic_inverse_matrix = basic_inverse_matrix.into_symmetric();

        Ok(ExtendedMatrix { tolerance: self.tolerance, basic_matrix: basic_inverse_matrix })
    }


    pub fn remove_zeros_rows_columns(&mut self) -> Vec<MatrixElementPosition<T>>
    {
        self.basic_matrix.remove_zeros_rows_columns()
    }


    pub fn remove_selected_row(&mut self, row: T)
    {
        self.basic_matrix = self.basic_matrix.remove_selected_row(row);
    }


    pub fn remove_selected_column(&mut self, column: T)
    {
        self.basic_matrix = self.basic_matrix.remove_selected_column(column);
    }


    pub fn define_type(&self) -> BasicMatrixType
    {
        self.basic_matrix.define_type()
    }
}
