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
    conversion_uint_into_usize,
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
             Ord + 'static,
          V: Copy + Debug + PartialEq + AddAssign + MulAssign + Mul<Output = V> + Div<Output = V> +
             SubAssign + Sub<Output = V> + Add<Output = V> + Into<f64> + From<f32> + 'static,
{
    pub fn create(rows_number: T, columns_number: T, all_elements_values: Vec<V>, tolerance: V)
        -> Result<Self, String>
    {
        let basic_matrix = BasicMatrix::create(rows_number, columns_number,
            all_elements_values, tolerance)?;
        Ok(NewExtendedMatrix { tolerance, basic_matrix })
    }


    fn matrices_dimensions_conformity_check(&self, other: &NewExtendedMatrix<T, V>,
        operation: Operation) -> Result<(T, Shape<T>), String>
    {
        let lhs_shape = self.copy_shape();
        let rhs_shape = other.copy_shape();
        match operation
        {
            Operation::Multiplication =>
                {
                    if lhs_shape.1 != rhs_shape.0
                    {
                        return Err("Extended matrix: Shapes of matrices \
                            does not conform to each other!".to_string());
                    }
                    Ok((lhs_shape.1, Shape(lhs_shape.0, rhs_shape.1)))
                },
            Operation::Addition =>
                {
                    if lhs_shape.0 != rhs_shape.0 || lhs_shape.1 != rhs_shape.1
                    {
                        return Err("Extended matrix: Shapes of matrices \
                            does not conform to each other!".to_string());
                    }
                    Ok((lhs_shape.1, Shape(lhs_shape.0, rhs_shape.1)))
                }
            Operation::Subtraction =>
                {
                    if lhs_shape.0 != rhs_shape.0 || lhs_shape.1 != rhs_shape.1
                    {
                        return Err("Extended matrix: Shapes of matrices \
                            does not conform to each other!".to_string());
                    }
                    Ok((lhs_shape.1, Shape(lhs_shape.0, rhs_shape.1)))
                }
        }
    }


    pub fn add_subtract_matrix(&self, other: &Self, operation: Operation) -> Result<Self, String>
    {
        let (_, shape) = self.matrices_dimensions_conformity_check(&other, operation)?;

        let basic_matrix_type =
            {
                if *self.ref_matrix_type() == BasicMatrixType::Symmetric &&
                    *other.ref_matrix_type() == BasicMatrixType::Symmetric
                {
                    BasicMatrixType::Symmetric
                }
                else
                {
                    BasicMatrixType::NonSymmetric
                }
            };

        let mut basic_matrix = BasicMatrix::create_default(shape.0,
            shape.1, basic_matrix_type);

        let mut row = T::from(0u8);
        while row < shape.0
        {
            let mut column = T::from(0u8);
            {
                while column < shape.1
                {
                    let mut current_matrix_element_position =
                        MatrixElementPosition::create(row, column);
                    let current_lhs_element_value =
                        self.basic_matrix.copy_element_value_or_zero(
                            current_matrix_element_position.clone())?;
                    let current_rhs_element_value =
                        other.basic_matrix.copy_element_value_or_zero(
                            current_matrix_element_position)?;
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
                                        not be applied for add_subtract function!".to_string()),
                            }
                        };

                    let matrix_element_position =
                        MatrixElementPosition::create(row, column);
                    basic_matrix.insert_matrix_element(matrix_element_position, element_value,
                        self.tolerance);

                    column += T::from(1u8);
                }
            }
            row += T::from(1u8);
        }

        Ok(NewExtendedMatrix { tolerance: self.tolerance, basic_matrix })
    }


    pub fn add_matrix(&self, other: &Self) -> Result<Self, String>
    {
        self.add_subtract_matrix(other, Operation::Addition)
    }


    pub fn subtract_matrix(&self, other: &Self) -> Result<Self, String>
    {
        self.add_subtract_matrix(other, Operation::Subtraction)
    }


    pub fn multiply_by_number(&mut self, number: V)
    {
        self.basic_matrix.multiply_by_number(number);
    }


    pub fn multiply_by_matrix(&self, other: &Self) -> Result<Self, String>
    {
        let (basic_dimension, shape) = self.matrices_dimensions_conformity_check(
            &other, Operation::Multiplication)?;

        let basic_matrix_type =
            {
                if *self.ref_matrix_type() == BasicMatrixType::Symmetric &&
                    *other.ref_matrix_type() == BasicMatrixType::Symmetric
                {
                    BasicMatrixType::Symmetric
                }
                else
                {
                    BasicMatrixType::NonSymmetric
                }
            };

        let mut basic_matrix = BasicMatrix::create_default(shape.0,
            shape.1, basic_matrix_type);

        let mut index = T::from(0u8);
        while index < shape.0 * shape.1
        {
            let mut element_value = V::from(0f32);

            let mut k = T::from(0u8);
            while k < basic_dimension
            {
                let mut current_lhs_matrix_element_position =
                    MatrixElementPosition::create(index / shape.1, k);
                let current_lhs_element_value = self.basic_matrix.copy_element_value_or_zero(
                    current_lhs_matrix_element_position)?;

                let mut current_rhs_matrix_element_position =
                    MatrixElementPosition::create(k, index % shape.1);
                let current_rhs_element_value = other.basic_matrix.copy_element_value_or_zero(
                        current_rhs_matrix_element_position)?;
                element_value += current_lhs_element_value * current_rhs_element_value;
                k += T::from(1u8);
            }

            let matrix_element_position =
                MatrixElementPosition::create(index / shape.1, index % shape.1);

            basic_matrix.insert_matrix_element(matrix_element_position, element_value,
                self.tolerance);

            index += T::from(1u8);
        }
        Ok(NewExtendedMatrix { tolerance: self.tolerance, basic_matrix })
    }


    pub fn transpose(&mut self)
    {
        self.basic_matrix.transpose();
    }


    pub fn direct_solution(&self, other: &Self) -> Result<Self, String>
    {
        let (basic_dimension, shape) = self.matrices_dimensions_conformity_check(
            &other, Operation::Multiplication)?;

        let mut lhs_matrix = self.clone();
        let mut rhs_matrix = other.clone();

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
                let current_lhs_element_value =
                    lhs_matrix.basic_matrix.copy_element_value_or_zero(
                    MatrixElementPosition::create(i, k))?;

                let current_diag_lhs_element_value =
                    lhs_matrix.basic_matrix.copy_element_value_or_zero(
                        MatrixElementPosition::create(k, k))?;

                if current_diag_lhs_element_value == V::from(0f32)
                {
                    return Err("Extended matrix: Matrix is singular!".to_string());
                }

                let current_coefficient =
                    current_lhs_element_value / current_diag_lhs_element_value;

                let mut j = k + T::from(1u8);
                while j < basic_dimension
                {
                    let current_lhs_element_value =
                        lhs_matrix.basic_matrix.copy_element_value_or_zero(
                            MatrixElementPosition::create(k, j))?;

                    lhs_matrix.basic_matrix.add_sub_mul_assign_matrix_element_value(
                        MatrixElementPosition::create(i, j),
                        current_coefficient * current_lhs_element_value,
                        Operation::Subtraction);
                    j += T::from(1u8);
                }

                let current_rhs_element_value =
                    rhs_matrix.basic_matrix.copy_element_value_or_zero(
                        MatrixElementPosition::create(k, T::from(0u8)))?;

                rhs_matrix.basic_matrix.add_sub_mul_assign_matrix_element_value(
                    MatrixElementPosition::create(i, T::from(0u8)),
                    current_coefficient * current_rhs_element_value,
                    Operation::Subtraction);

                i += T::from(1u8);
            }
            k += T::from(1u8);
        }

        let rhs_element_value = rhs_matrix.basic_matrix.copy_element_value_or_zero(
            MatrixElementPosition::create(basic_dimension - T::from(1u8),
            T::from(0u8)))?;

        let lhs_element_value = lhs_matrix.basic_matrix.copy_element_value_or_zero(
            MatrixElementPosition::create(basic_dimension - T::from(1u8),
            basic_dimension - T::from(1u8)))?;

        let n = conversion_uint_into_usize(basic_dimension - T::from(1u8));

        elements_values[n] = rhs_element_value / lhs_element_value;

        let mut i = basic_dimension - T::from(1u8);
        while i > T::from(0u8)
        {
            i -= T::from(1u8);

            let rhs_element_value = rhs_matrix.basic_matrix.copy_element_value_or_zero(
                MatrixElementPosition::create(i, T::from(0u8)))?;

            let mut sum = rhs_element_value;

            let mut j = i + T::from(1u8);
            while j < basic_dimension
            {

                let lhs_element_value = lhs_matrix.basic_matrix.copy_element_value_or_zero(
                    MatrixElementPosition::create(i, j))?;

                let n = conversion_uint_into_usize(j);

                sum -= lhs_element_value * elements_values[n];
                j += T::from(1u8);
            }

            let lhs_element_value = lhs_matrix.basic_matrix.copy_element_value_or_zero(
                MatrixElementPosition::create(i, i))?;

            let n = conversion_uint_into_usize(i);

            elements_values[n] = sum / lhs_element_value;
        }

        Ok(NewExtendedMatrix::create(shape.0, shape.1,
            elements_values, self.tolerance)?)
    }


    pub fn lu_decomposition(&self) -> Result<(Self, Self), String>
    {
        let shape = self.copy_shape();
        if (shape.0 != shape.1) || shape.0 < T::from(2u8)
        {
            return Err("Extended matrix: Matrix could not be decomposed!".to_string());
        }

        let mut origin_matrix = self.clone();
        origin_matrix.into_nonsymmetric();

        let mut l_basic_matrix = BasicMatrix::create_default(
            shape.0, shape.1, BasicMatrixType::NonSymmetric);
        let mut u_basic_matrix = BasicMatrix::create_default(
            shape.0, shape.1, BasicMatrixType::NonSymmetric);

        let mut i = T::from(0u8);
        while i < shape.0
        {
            l_basic_matrix.insert_matrix_element(MatrixElementPosition::create(
                i, i), V::from(1f32), self.tolerance);
            i += T::from(1u8);
        }

        let mut k = T::from(0u8);
        while k < shape.1
        {
            let current_element_value = origin_matrix.basic_matrix.copy_element_value_or_zero(
                MatrixElementPosition::create(T::from(0u8), k))?;

            u_basic_matrix.insert_matrix_element(MatrixElementPosition::create(
                T::from(0u8), k), current_element_value, self.tolerance);

            k += T::from(1u8);
        }

        let mut row_number = T::from(0u8);

        while row_number < shape.0 - T::from(1u8)
        {
            let mut i = row_number + T::from(1u8);
            while i < shape.0
            {
                let current_coefficient = origin_matrix.basic_matrix.copy_element_value_or_zero(
                    MatrixElementPosition::create(i, row_number))? /
                    origin_matrix.basic_matrix.copy_element_value_or_zero(
                        MatrixElementPosition::create(
                            row_number, row_number))?;

                l_basic_matrix.insert_matrix_element(
                    MatrixElementPosition::create(i, row_number),
                    current_coefficient, self.tolerance);

                let mut j = T::from(0u8);
                while j < shape.1
                {
                    let current_element_value =
                        origin_matrix.basic_matrix.copy_element_value_or_zero(
                            MatrixElementPosition::create(i, j))? -
                        origin_matrix.basic_matrix.copy_element_value_or_zero(
                            MatrixElementPosition::create(row_number,
                                j))? * current_coefficient;

                    if current_element_value.into().abs() > self.tolerance.into()
                    {
                         u_basic_matrix.insert_matrix_element(
                            MatrixElementPosition::create(i, j),
                            current_element_value, self.tolerance);
                        origin_matrix.basic_matrix.insert_matrix_element(
                            MatrixElementPosition::create(i, j),
                            current_element_value, self.tolerance);
                    }
                    else
                    {
                        u_basic_matrix.remove_matrix_element(
                            MatrixElementPosition::create(i, j));
                        origin_matrix.basic_matrix.remove_matrix_element(
                            MatrixElementPosition::create(i, j));
                    }

                    j += T::from(1u8);
                }
                i += T::from(1u8);
            }
            row_number += T::from(1u8);
        }

        let l_matrix = NewExtendedMatrix { tolerance: self.tolerance, basic_matrix: l_basic_matrix };
        let u_matrix = NewExtendedMatrix { tolerance: self.tolerance, basic_matrix: u_basic_matrix };
        Ok((l_matrix, u_matrix))
    }


    pub fn determinant(&self) -> Result<V, String>
    {
        let (_, u_matrix) = self.lu_decomposition()?;
        let shape = u_matrix.copy_shape();
        let mut determinant = V::from(1f32);

        let mut i = T::from(0u8);
        while i < shape.0
        {
            let current_diag_element_value = u_matrix.basic_matrix.copy_element_value_or_zero(
                MatrixElementPosition::create(i, i))?;
            determinant *= current_diag_element_value;
            i += T::from(1u8);
        }

        Ok(determinant)
    }


    pub fn inverse(&self) -> Result<Self, String>
    {
        let (l_matrix, u_matrix) =
            self.lu_decomposition()?;

        let f = |data: &str| println!("{}", data);

        let shape = self.basic_matrix.copy_shape();

        let mut basic_inverse_matrix = BasicMatrix::create_default(
            shape.0, shape.1, BasicMatrixType::NonSymmetric);

        let mut k = T::from(0u8);
        while k < shape.1
        {
            let mut basic_unit_column = BasicMatrix::create_default(
                shape.1, T::from(1u8),
                BasicMatrixType::NonSymmetric);

            basic_unit_column.insert_matrix_element(
                MatrixElementPosition::create(k, T::from(0u8)),
                V::from(1f32), self.tolerance);

            let unit_column =
                NewExtendedMatrix { tolerance: self.tolerance, basic_matrix: basic_unit_column };

            let interim_inverse_column =
                l_matrix.direct_solution(&unit_column)?;

            let inverse_column =
                u_matrix.direct_solution(&interim_inverse_column)?;

            let mut i = T::from(0u8);
            while i < shape.0
            {
                let current_inverse_column_element_value =
                    inverse_column.basic_matrix.copy_element_value_or_zero(
                        MatrixElementPosition::create(i,
                        T::from(0u8)))?;

                basic_inverse_matrix.insert_matrix_element(
                    MatrixElementPosition::create(i, k),
                    current_inverse_column_element_value, self.tolerance);

                i += T::from(1u8);
            }
            k += T::from(1u8);
        }

        Ok(NewExtendedMatrix { tolerance: self.tolerance, basic_matrix: basic_inverse_matrix })
    }


    pub fn add_submatrix_to_assemblage(&mut self, submatrix: &mut Self,
        assemblage_positions: &[MatrixElementPosition<T>],
        submatrix_positions: &[MatrixElementPosition<T>])
    {
        if self.ref_matrix_type() != submatrix.ref_matrix_type()
        {
            self.into_nonsymmetric();
            submatrix.into_nonsymmetric();
        }

        for (lhs_position, rhs_position) in
            assemblage_positions.iter().zip(submatrix_positions.iter())
        {

            if let Some(rhs_element_value) =
                submatrix.basic_matrix.ref_elements_values().get(rhs_position)
            {
                self.basic_matrix.add_sub_mul_assign_matrix_element_value(
                    lhs_position.clone(),
                    *rhs_element_value, Operation::Addition);
            }
        }
    }


    pub fn try_to_symmetrize(&mut self)
    {
        self.basic_matrix.try_to_symmetrize();
    }


    pub fn into_nonsymmetric(&mut self)
    {
        self.basic_matrix.into_nonsymmetric();
    }


    pub fn remove_zeros_rows_columns(&mut self) -> Vec<MatrixElementPosition<T>>
    {
        self.basic_matrix.remove_zeros_rows_columns()
    }


    pub fn remove_selected_row(&mut self, row: T)
    {
        self.basic_matrix.remove_selected_row(row);
    }


    pub fn remove_selected_column(&mut self, column: T)
    {
        self.basic_matrix.remove_selected_column(column);
    }


    pub fn copy_shape(&self) -> Shape<T>
    {
        self.basic_matrix.copy_shape()
    }


    pub fn ref_matrix_type(&self) -> &BasicMatrixType
    {
        self.basic_matrix.ref_matrix_type()
    }


    pub fn clone_all_elements_values(&self) -> HashMap<MatrixElementPosition<T>, V>
    {
        self.basic_matrix.clone_all_elements_values()
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
                let mut matrix_element_position =
                    MatrixElementPosition::create(row, column);
                row_str += &format!("{:?}, ",
                    self.basic_matrix.copy_element_value_or_zero(matrix_element_position).unwrap());
                column += T::from(1u8);
            }
            f(&format!("{}", row_str));
            row += T::from(1u8);
        }
    }
}
























//
//
//     pub fn define_type(&self) -> BasicMatrixType
//     {
//         self.basic_matrix.define_type()
//     }
// }
