use std::ops::{Mul, Add, Sub, Div, Rem, MulAssign, SubAssign};
use std::fmt::Debug;
use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;

use crate::basic_matrix::basic_matrix::BasicMatrixTrait;
use crate::basic_matrix::basic_matrix::{MatrixElementPosition, Shape};
use crate::basic_matrix::basic_matrix::{BasicMatrixType};

use crate::basic_matrix::symmetric_matrix::SymmetricMatrix;

use crate::basic_matrix::functions::{matrix_size_check, copy_value_by_index};


#[derive(Debug, Clone)]
pub struct NonSymmetricMatrix<T, V>
    where T: Copy + Debug,
          V: Copy
{
    rows_number: T,
    columns_number: T,
    elements_indexes: Vec<T>,
    elements_values: Vec<V>,
}


impl<T, V> BasicMatrixTrait<T, V> for NonSymmetricMatrix<T, V>
    where T: Copy + PartialEq + Debug + PartialOrd + Mul<Output = T> + Add<Output = T> +
             Sub<Output = T> + Div<Output = T> + Rem<Output = T> + Eq + Hash + SubAssign +
             From<u8> + 'static,
          V: Copy + PartialEq + Debug + MulAssign + From<f32> + 'static,
{
    fn read_element_value(&self, row: T, column: T) -> Result<V, &str>
    {
        matrix_size_check(
            row, column,
            (self.rows_number, self.columns_number))?;
        let requested_index = row * self.columns_number + column;
        let value = copy_value_by_index(
            requested_index,
            self.elements_indexes.as_slice(),
            self.elements_values.as_slice());
        Ok(value)
    }


    fn copy_all_elements_values(&self) -> HashMap<MatrixElementPosition<T>, V>
    {
        let mut all_elements_values = HashMap::new();
        for (index, value) in self.elements_indexes.iter()
            .zip(self.elements_values.iter())
        {
            let row = *index / self.columns_number;
            let column = *index % self.columns_number;
            let position = MatrixElementPosition::create(row, column);
            all_elements_values.insert(position, *value);
        }
        all_elements_values
    }


    fn copy_shape(&self) -> Shape<T>
    {
        Shape(self.rows_number, self.columns_number)
    }


    fn transpose(&mut self)
    {
        let transposed_rows_number = self.columns_number;
        let transposed_columns_number = self.rows_number;
        for i in 0..self.elements_indexes.len()
        {
            let current_index_row = self.elements_indexes[i] / self.columns_number;
            let current_index_column = self.elements_indexes[i] % self.columns_number;
            let transported_index_row = current_index_column;
            let transported_index_column = current_index_row;
            let transported_index = transported_index_row * transposed_columns_number +
                transported_index_column;
            self.elements_indexes[i] = transported_index;
        }
        self.rows_number = transposed_rows_number;
        self.columns_number = transposed_columns_number;
    }


    fn multiply_by_number(&mut self, number: V)
    {
        for i in 0..self.elements_values.len()
        {
            self.elements_values[i] *= number;
        }
    }


    fn into_symmetric(self) -> Box<dyn BasicMatrixTrait<T, V>>
    {
        if self.rows_number != self.columns_number
        {
            return Box::new(self);
        }
        let mut elements_indexes = Vec::new();
        let mut elements_values = Vec::new();
        let mut indexes = self.elements_indexes.clone();
        let mut values = self.elements_values.clone();
        while !indexes.is_empty()
        {
            let current_index = indexes.remove(0);
            let current_value = values.remove(0);
            let current_row = current_index / self.rows_number;
            let current_column = current_index % self.rows_number;
            if current_row == current_column
            {
                elements_indexes.push(current_index);
                elements_values.push(current_value);
                continue;
            }
            if let Some(position) = indexes
                .iter()
                .position(|index| *index == current_column * self.rows_number + current_row)
            {
                let current_symmetric_value = values.remove(position);
                if current_value != current_symmetric_value
                {
                    return Box::new(self);
                }
                indexes.remove(position);
                let (row, column) =
                    if current_row < current_column { (current_row, current_column) }
                    else { (current_column, current_row) };
                elements_indexes.push(row * self.rows_number + column);
                elements_values.push(current_value);
            }
            else
            {
                return Box::new(self);
            }
        }
        Box::new(SymmetricMatrix::create(self.rows_number, elements_indexes,
            elements_values))
    }


    fn define_type(&self) -> BasicMatrixType
    {
        BasicMatrixType::NonSymmetric
    }


    fn remove_zeros_rows_columns(&mut self) -> Vec<MatrixElementPosition<T>>
    {
        let mut zeros_rows_columns = Vec::new();
        let mut can_continue = true;
        while can_continue
        {
            if let Some(row) = self.find_zeros_row()
            {
                if let Some(column) = self.find_zeros_column()
                {
                    let zeros_row_column = MatrixElementPosition::create(row, column);
                    zeros_rows_columns.push(zeros_row_column);
                    self.remove_row(row);
                    self.remove_column(column);
                }
                else
                {
                    can_continue = false;
                }
            }
            else
            {
                can_continue = false;
            }
        }
        zeros_rows_columns
    }


    fn remove_selected_row(&mut self, row: T) -> Box<dyn BasicMatrixTrait<T, V>>
    {
        self.remove_row(row);
        Box::new(self.clone())
    }


    fn remove_selected_column(&mut self, column: T) -> Box<dyn BasicMatrixTrait<T, V>>
    {
        self.remove_column(column);
        Box::new(self.clone())
    }


    fn as_any(&self) -> &dyn Any
    {
        self
    }
}


impl<T, V> NonSymmetricMatrix<T, V>
    where T: Copy + Debug + PartialEq + From<u8> + Mul<Output = T> + Add<Output = T> + PartialOrd +
             SubAssign + Div<Output = T> + Sub<Output = T> + Rem<Output = T>,
          V: Copy
{
    pub fn create(rows_number: T, columns_number: T, elements_indexes: Vec<T>,
        elements_values: Vec<V>) -> Self
    {
        NonSymmetricMatrix { rows_number, columns_number, elements_indexes, elements_values }
    }


    pub fn ref_rows_number(&self) -> &T
    {
        &self.rows_number
    }


    pub fn ref_columns_number(&self) -> &T
    {
        &self.columns_number
    }


    pub fn ref_elements_indexes(&self) -> &[T]
    {
        self.elements_indexes.as_slice()
    }


    pub fn ref_elements_values(&self) -> &[V]
    {
        self.elements_values.as_slice()
    }


    fn find_zeros_row(&self) -> Option<T>
    {
        let mut zeros_row = None;
        let find_index = |row, column| self.elements_indexes
            .iter()
            .position(|index|
                {
                    *index == row * self.columns_number + column
                });
        let mut row = self.rows_number;
        while row > T::from(0u8)
        {
            row -= T::from(1u8);
            let mut answers = Vec::new();
            let mut column = self.columns_number;
            while column > T::from(0u8)
            {
                column -= T::from(1u8);
                match find_index(row, column)
                {
                    None => answers.push(true),
                    Some(_) => answers.push(false),
                }
            }
            if answers.iter().all(|answer| *answer == true)
            {
                zeros_row = Some(row);
            }
            if zeros_row != None
            {
                break;
            }
        }
        zeros_row
    }


    fn find_zeros_column(&self) -> Option<T>
    {
        let mut zeros_column = None;
        let find_index = |row, column| self.elements_indexes
            .iter()
            .position(|index| *index == row * self.columns_number + column);

        let mut column = self.columns_number;
        while column > T::from(0u8)
        {
            column -= T::from(1u8);
            let mut answers = Vec::new();
            let mut row = self.rows_number;
            while row > T::from(0u8)
            {
                row -= T::from(1u8);
                match find_index(row, column)
                {
                    None => answers.push(true),
                    Some(_) => answers.push(false),
                }
            }
            if answers.iter().all(|answer| *answer == true)
            {
                zeros_column = Some(column);
            }
            if zeros_column != None
            {
                break;
            }
        }
        zeros_column
    }


    pub fn remove_row(&mut self, row: T)
    {
        for index in self.elements_indexes.as_mut_slice()
        {
            if *index > row * self.columns_number
            {
                *index -= self.columns_number;
            }
        }
        self.rows_number -= T::from(1u8);
    }


    pub fn remove_column(&mut self, column: T)
    {
        for index in self.elements_indexes.as_mut_slice()
        {
            if *index % self.columns_number > column
            {
                *index -= *index / self.columns_number + T::from(1u8);
            }
            else
            {
                *index -= *index / self.columns_number;
            }
        }
        self.columns_number -= T::from(1u8);
    }
}
