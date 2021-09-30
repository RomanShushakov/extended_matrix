use std::fmt::Debug;
use std::convert::{From, Into};
use std::ops::{Mul, Add, Sub, Div, Rem, MulAssign, AddAssign, SubAssign};
use std::hash::Hash;
use std::collections::HashMap;

use crate::basic_matrix::basic_matrix::{BasicMatrix, BasicMatrixTrait, BasicMatrixType};
use crate::basic_matrix::basic_matrix::{MatrixElementPosition, Shape};

use crate::basic_matrix::non_symmetric_matrix::NonSymmetricMatrix;

use crate::functions::
{
    new_matrices_dimensions_conformity_check, copy_element_value_or_zero,
    conversion_uint_into_usize
};


#[derive(Copy, Clone)]
pub enum Operation
{
    Addition,
    Multiplication,
    Subtraction,
}


#[derive(Clone)]
pub struct NewExtendedMatrix<T, V>
{
    tolerance: V,
    basic_matrix: BasicMatrix<T, V>
}


impl<T, V> NewExtendedMatrix<T, V>
    where T: Copy + Debug + Mul<Output = T> + PartialOrd + Add<Output = T> + Sub<Output = T> +
             Div<Output = T> + Rem<Output = T> + Eq + Hash + SubAssign + AddAssign + From<u8> +
             'static,
          V: Copy + Debug + PartialEq + AddAssign + MulAssign + Mul<Output = V> + Div<Output = V> +
             SubAssign + Sub<Output = V> + Add<Output = V> + Into<f64> + From<f32> + 'static,
{
    pub fn create(rows_number: T, columns_number: T, all_elements_values: Vec<V>, tolerance: V)
                  -> Result<Self, &'static str>
    {
        let basic_matrix = BasicMatrix::create(rows_number, columns_number,
            all_elements_values, tolerance)?;
        Ok(NewExtendedMatrix { tolerance, basic_matrix })
    }


    pub fn create_default(rows_number: T, columns_number: T, tolerance: V) -> Self
    {
        let basic_matrix = BasicMatrix::create_default(rows_number, columns_number);
        NewExtendedMatrix { tolerance, basic_matrix }
    }


    pub fn copy_element_value_or_zero(&self, row: T, column: T) -> Result<V, &str>
    {
        self.basic_matrix.copy_element_value_or_zero(row, column)
    }


    pub fn add_subtract_matrix<'a>(&'a self, other: &'a Self, operation: Operation)
        -> Result<Self, &'a str>
    {
        let (_, shape) =
            new_matrices_dimensions_conformity_check(&self, &other, operation)?;

        let mut basic_matrix =
            BasicMatrix::create_default(shape.0, shape.1);

        let mut row = T::from(0u8);
        while row < shape.0
        {
            let mut column = T::from(0u8);
            {
                while column < shape.1
                {
                    let current_lhs_element_value =
                        self.copy_element_value_or_zero(row, column)?;
                    let current_rhs_element_value =
                        other.copy_element_value_or_zero(row, column)?;
                    let element_value =
                        {
                            match operation
                            {
                                Operation::Addition =>
                                    current_lhs_element_value + current_rhs_element_value,
                                Operation::Subtraction =>
                                    current_lhs_element_value - current_rhs_element_value,
                                Operation::Multiplication =>
                                    return Err("Extended matrix: Multiplication operation could \
                                        not be applied for add_subtract function!"),
                            }
                        };
                    if element_value.into().abs() > self.tolerance.into()
                    {
                        let matrix_element_position =
                            MatrixElementPosition::create(row, column);
                        basic_matrix.add_value_to_matrix_element(matrix_element_position,
                            element_value);
                    }

                    column += T::from(1u8);
                }
            }
            row += T::from(1u8);
        }

        Ok(NewExtendedMatrix { tolerance: self.tolerance, basic_matrix })
    }


    pub fn add_matrix<'a>(&'a self, other: &'a Self) -> Result<Self, &'a str>
    {
        self.add_subtract_matrix(other, Operation::Addition)
    }


    pub fn subtract_matrix<'a>(&'a self, other: &'a Self) -> Result<Self, &'a str>
    {
        self.add_subtract_matrix(other, Operation::Subtraction)
    }


    pub fn multiply_by_number(&mut self, number: V)
    {
        self.basic_matrix.multiply_by_number(number);
    }


    pub fn multiply_by_matrix<'a>(&'a self, other: &'a Self)
        -> Result<Self, &'a str>
    {
        let (basic_dimension, shape) = new_matrices_dimensions_conformity_check(
            &self, &other, Operation::Multiplication)?;

        let mut basic_matrix = BasicMatrix::create_default(shape.0,
            shape.1);

        let mut index = T::from(0u8);
        while index < shape.0 * shape.1
        {
            let mut element_value = V::from(0f32);

            let mut k = T::from(0u8);
            while k < basic_dimension
            {
                let current_lhs_element_value = self.copy_element_value_or_zero(
                    index / shape.1, k)?;
                let current_rhs_element_value = other.copy_element_value_or_zero(
                        k, index % shape.1)?;
                element_value += current_lhs_element_value * current_rhs_element_value;
                k += T::from(1u8);
            }

            if element_value.into().abs() > self.tolerance.into()
            {
                let matrix_element_position =
                    MatrixElementPosition::create(index / shape.1, index % shape.1);
                basic_matrix.add_value_to_matrix_element(matrix_element_position, element_value);
            }
            index += T::from(1u8);
        }
        Ok(NewExtendedMatrix { tolerance: self.tolerance, basic_matrix })
    }


    pub fn transpose(&mut self)
    {
        self.basic_matrix.transpose();
    }


    pub fn add_submatrix_to_assemblage(&mut self, submatrix: &Self,
        assemblage_positions: &[MatrixElementPosition<T>],
        submatrix_positions: &[MatrixElementPosition<T>])
    {
        for (lhs_position, rhs_position) in
            assemblage_positions.iter().zip(submatrix_positions)
        {
            if let Some(rhs_element_value) =
                submatrix.ref_elements_values().get(rhs_position)
            {
                self.basic_matrix.add_value_to_matrix_element(
                    lhs_position.clone(), *rhs_element_value);
            }
        }
    }


    fn add_value_to_matrix_element(&mut self, matrix_element_position: MatrixElementPosition<T>,
        element_value: V)
    {
        self.basic_matrix.add_value_to_matrix_element(matrix_element_position, element_value)
    }


    pub fn naive_gauss_elimination<'a>(&'a self, other: &'a Self) -> Result<Self, &'a str>
    {
        let (basic_dimension, shape) = new_matrices_dimensions_conformity_check(
            &self, &other, Operation::Multiplication)?;

        let mut lhs_all_elements_values =
            self.clone_elements_values();
        let mut rhs_all_elements_values =
            other.clone_elements_values();

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

        Ok(NewExtendedMatrix::create(shape.0, shape.1,
            elements_values, self.tolerance)?)
    }


    pub fn lu_decomposition(&self) -> Result<(Self, Self), &str>
    {
        let shape = self.copy_shape();
        if (shape.0 != shape.1) || shape.0 < T::from(2u8)
        {
            return Err("Extended matrix: Matrix could not be decomposed!");
        }

        let mut l_matrix = NewExtendedMatrix::create_default(
            shape.0, shape.1, self.tolerance);
        let mut u_matrix = NewExtendedMatrix::create_default(
            shape.0, shape.1, self.tolerance);

        let mut l_elements_indexes = Vec::new();
        let mut l_elements_values= Vec::new();

        let mut i = T::from(0u8);
        while i < shape.0
        {
            l_matrix.add_value_to_matrix_element(MatrixElementPosition::create(
                i * shape.1, i), V::from(1f32));

            l_elements_indexes.push(i * shape.1 + i);
            l_elements_values.push(V::from(1f32));
            i += T::from(1u8);
        }

        let mut all_elements_values =
            self.clone_elements_values();
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

        println!("{:?}, {:?}", l_elements_values, l_elements_indexes);

        let l_matrix = NewExtendedMatrix::create(shape.0,
            shape.1, l_elements_values, self.tolerance)?;

        let u_matrix = NewExtendedMatrix::create(shape.0,
            shape.1, u_elements_values, self.tolerance)?;

        Ok((l_matrix, u_matrix))
    }


    pub fn determinant(&self) -> Result<V, &str>
    {
        let (_, u_matrix) = self.lu_decomposition()?;
        let u_matrix_elements_values =
            u_matrix.clone_elements_values();
        let shape = u_matrix.copy_shape();
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



        let shape = self.copy_shape();

        let mut inverse_matrix = NewExtendedMatrix::create_default(
            shape.0, shape.1, self.tolerance);

        let mut k = T::from(0u8);
        while k < shape.1
        {
            let unit_column_values = vec![V::from(1f32)];

            // let unit_column = NewExtendedMatrix::create(shape.1,
            //     T::from(1u8), unit_column_values, self.tolerance)?;



            let mut unit_column = NewExtendedMatrix::create_default(
                shape.1, T::from(1u8), self.tolerance);



            let interim_inverse_column = l_matrix
                .naive_gauss_elimination(&unit_column).unwrap();

            let inverse_column = u_matrix
                .naive_gauss_elimination(&interim_inverse_column).unwrap();

            let all_inverse_column_values =
                inverse_column.clone_elements_values();

            let mut i = T::from(0u8);
            while i < shape.0
            {
                let current_inverse_column_element_value = copy_element_value_or_zero(i,
                    T::from(0u8), &all_inverse_column_values);

                 if current_inverse_column_element_value != V::from(0f32)
                {
                    inverse_matrix.add_value_to_matrix_element(
                        MatrixElementPosition::create(i * shape.1, k),
                        current_inverse_column_element_value);
                }

                i += T::from(1u8);
            }
            k += T::from(1u8);
        }

        Ok(inverse_matrix)
    }


    pub fn try_into_symmetric(&mut self) -> Result<(), &str>
    {
        self.basic_matrix.try_into_symmetric()
    }


    pub fn copy_shape(&self) -> Shape<T>
    {
        self.basic_matrix.copy_shape()
    }


    pub fn ref_matrix_type(&self) -> &BasicMatrixType
    {
        self.basic_matrix.ref_matrix_type()
    }


    pub fn ref_elements_values(&self) -> &HashMap<MatrixElementPosition<T>, V>
    {
        self.basic_matrix.ref_elements_values()
    }


    pub fn clone_elements_values(&self) -> HashMap<MatrixElementPosition<T>, V>
    {
        self.basic_matrix.clone_elements_values()
    }


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
                    self.copy_element_value_or_zero(row, column).unwrap());
                column += T::from(1u8);
            }
            f(&format!("{}", row_str));
            row += T::from(1u8);
        }
    }
}























//
//
//     pub fn remove_zeros_rows_columns(&mut self) -> Vec<MatrixElementPosition<T>>
//     {
//         self.basic_matrix.remove_zeros_rows_columns()
//     }
//
//
//     pub fn remove_selected_row(&mut self, row: T)
//     {
//         self.basic_matrix = self.basic_matrix.remove_selected_row(row);
//     }
//
//
//     pub fn remove_selected_column(&mut self, column: T)
//     {
//         self.basic_matrix = self.basic_matrix.remove_selected_column(column);
//     }
//
//
//     pub fn define_type(&self) -> BasicMatrixType
//     {
//         self.basic_matrix.define_type()
//     }
// }
