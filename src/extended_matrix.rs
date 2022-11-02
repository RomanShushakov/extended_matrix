use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

use colsol::{factorization, find_unknown};

use crate::basic_matrix::basic_matrix::{BasicMatrix, BasicMatrixType};

use crate::matrix::Shape;
use crate::matrix_element_position::MatrixElementPosition;

use crate::functions::
{
    conversion_uint_into_usize, conversion_usize_into_uint, matrix_element_value_extractor, try_to_compact_matrix,
    decompose, substitute,
};

use crate::enums::Operation;
use crate::traits::{UIntTrait, FloatTrait};


#[derive(Clone, Debug)]
pub struct ExtendedMatrix<T, V>
{
    tolerance: V,
    basic_matrix: BasicMatrix<T, V>
}


impl<T, V> ExtendedMatrix<T, V>
    where T: UIntTrait<Output = T>,
          V: FloatTrait<Output = V>
{
    pub fn create(rows_number: T, columns_number: T, all_elements_values: Vec<V>, tolerance: V)
        -> Result<Self, String>
    {
        let basic_matrix = BasicMatrix::create(rows_number, columns_number,
            all_elements_values, tolerance)?;
        Ok(ExtendedMatrix { tolerance, basic_matrix })
    }


    pub(crate) fn matrices_dimensions_conformity_check(&self, other: &ExtendedMatrix<T, V>, operation: Operation) 
        -> Result<(T, Shape<T>), String>
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
                        do not conform to each other!".to_string());
                }
                Ok((lhs_shape.1, Shape(lhs_shape.0, rhs_shape.1)))
            },
            Operation::Addition | Operation::Subtraction =>
            {
                if lhs_shape.0 != rhs_shape.0 || lhs_shape.1 != rhs_shape.1
                {
                    return Err("Extended matrix: Shapes of matrices \
                        do not conform to each other!".to_string());
                }
                Ok((lhs_shape.1, Shape(lhs_shape.0, rhs_shape.1)))
            }
        }
    }


    fn add_subtract_matrix(&self, other: &Self, operation: Operation) -> Result<Self, String>
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
                    let current_matrix_element_position =
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

        Ok(ExtendedMatrix { tolerance: self.tolerance, basic_matrix })
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
                let current_lhs_matrix_element_position =
                    MatrixElementPosition::create(index / shape.1, k);
                let current_lhs_element_value = self.basic_matrix.copy_element_value_or_zero(
                    current_lhs_matrix_element_position)?;

                let current_rhs_matrix_element_position =
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
        Ok(ExtendedMatrix { tolerance: self.tolerance, basic_matrix })
    }


    pub fn transpose(&mut self)
    {
        self.basic_matrix.transpose();
    }


    pub fn direct_solution(&self, other: &Self, colsol_usage: bool) -> Result<Self, String>
    {
        let (basic_dimension, shape) = self.matrices_dimensions_conformity_check(
            &other, Operation::Multiplication)?;

        if *self.ref_matrix_type() == BasicMatrixType::Symmetric && colsol_usage
        {
            let (mut a, maxa) = try_to_compact_matrix(&self)?;
            let mut v = Vec::new();
            let mut nn = 0i64;
            let mut row = T::from(0u8);
            let column = T::from(0u8);
            while row < shape.0
            {
                let element_value = other.copy_element_value_or_zero(
                    MatrixElementPosition::create(row, column))?;
                v.push(element_value);
                nn += 1;
                row += T::from(1u8);
            }
            factorization::<V>(&mut a, nn, &maxa)?;
            find_unknown::<V>(&a, &mut v, nn, &maxa);
            return Ok(ExtendedMatrix::create(shape.0, shape.1,
                v, self.tolerance)?);
        }

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

        Ok(ExtendedMatrix::create(shape.0, shape.1,
            elements_values, self.tolerance)?)
    }


    pub fn lu_decomposition(&self) -> Result<Self, String>
    {
        let shape = self.copy_shape();

        if (shape.0 != shape.1) || shape.0 < T::from(2u8)
        {
            return Err(format!("Extended matrix: Could not decompose matrix! Rows number {:?} \
                does not match to columns number {:?}", shape.0, shape.1));
        }

        let mut decomposed_matrix = self.clone();
        decomposed_matrix.into_nonsymmetric();

        let n = conversion_uint_into_usize(shape.0);
        let mut o = vec![0usize; n];
        let mut s = vec![V::from(0f32); n];

        decompose(&mut decomposed_matrix, n, self.tolerance, &mut o, &mut s)?;

        Ok(decomposed_matrix)
    }


    pub(super) fn insert_matrix_element(&mut self, matrix_element_position: MatrixElementPosition<T>,
        element_value: V, tolerance: V)
    {   
        self.basic_matrix.insert_matrix_element(matrix_element_position, element_value, tolerance);
    }


    pub fn determinant(&self) -> Result<V, String>
    {
        let shape = self.copy_shape();
        if (shape.0 != shape.1) || shape.0 < T::from(2u8)
        {
            return Err(format!("Extended matrix: Could not find determinant of matrix! Rows number {:?} \
                does not match to columns number {:?}", shape.0, shape.1));
        }

        let decomposed_matrix = self.lu_decomposition()?;
        let shape = decomposed_matrix.copy_shape();
        let mut determinant = V::from(1f32);

        let mut i = T::from(0u8);
        while i < shape.0
        {
            let current_diag_element_value = decomposed_matrix.basic_matrix.copy_element_value_or_zero(
                MatrixElementPosition::create(i, i))?;
            determinant *= current_diag_element_value;
            i += T::from(1u8);
        }

        Ok(determinant)
    }


    pub fn determinant_2x2(&self) -> Result<V, String>
    {
        let shape = self.copy_shape();
        if (shape.0 != shape.1) || shape.0 != T::from(2u8)
        {
            return Err(format!("Extended matrix: Could not find determinant of matrix! Rows number: {:?}, \
                columns number: {:?}", shape.0, shape.1));
        }
        let determinant = 
            matrix_element_value_extractor(T::from(0u8), T::from(0u8), &self)? * 
            matrix_element_value_extractor(T::from(1u8), T::from(1u8), &self)? -
            matrix_element_value_extractor(T::from(0u8), T::from(1u8), &self)? *
            matrix_element_value_extractor(T::from(1u8), T::from(0u8), &self)?;
        Ok(determinant)
    }


    pub fn inverse(&self) -> Result<Self, String>
    {
        let shape = self.copy_shape();
        let n = conversion_uint_into_usize(shape.0);

        let mut a = self.clone();
        a.into_nonsymmetric();

        let mut o = vec![0usize; n];
        let mut s = vec![V::from(0f32); n];

        let mut b = vec![V::from(0f32); n];
        let mut ai = ExtendedMatrix::create(shape.0, shape.1, 
            vec![V::from(0f32); n * n], self.tolerance)?;
        ai.into_nonsymmetric();

        match decompose(&mut a, n, self.tolerance, &mut o, &mut s)
        {
            Ok(_) =>
                {
                    for i in 0..n
                    {
                        for j in 0..n
                        {
                            if i == j
                            {
                                b[j] = V::from(1f32);
                            }
                            else
                            {
                                b[j] = V::from(0f32);    
                            }
                        }
                        let mut x = vec![V::from(0f32); n];
                        substitute(&a, &o, n, &mut b, &mut x)?;
                        for j in 0..n
                        {
                            let matrix_element_position = MatrixElementPosition::create(
                                conversion_usize_into_uint(j), conversion_usize_into_uint(i));
                            ai.insert_matrix_element(matrix_element_position, x[j], self.tolerance);
                        }
                    }
                },
            Err(e) => 
                {
                    let error_message = format!("Extended matrix: Inverse matrix calculation: Ill conditioned system: {:?}", e);
                    return Err(error_message);
                }
        }
        Ok(ai)
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


    pub fn try_to_symmetrize(&mut self, tolerance: V)
    {
        self.basic_matrix.try_to_symmetrize(tolerance);
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


    pub fn copy_element_value_or_zero(&self, matrix_element_position: MatrixElementPosition<T>)
        -> Result<V, String>
    {
        self.basic_matrix.copy_element_value_or_zero(matrix_element_position)
    }


    pub fn ref_elements_values(&self) -> &HashMap<MatrixElementPosition<T>, V>
    {
        self.basic_matrix.ref_elements_values()
    }


    // pub fn clone_all_elements_values(&self) -> HashMap<MatrixElementPosition<T>, V>
    // {
    //     self.basic_matrix.clone_all_elements_values()
    // }


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
                let matrix_element_position =
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


impl<T, V> PartialEq for ExtendedMatrix<T, V>
    where T: PartialEq + Eq + Hash,
          V: PartialEq
{
    fn eq(&self, other: &Self) -> bool 
    {
        if self.tolerance != other.tolerance || self.basic_matrix != other.basic_matrix
        {
            return false;
        }
        true
    }
}
