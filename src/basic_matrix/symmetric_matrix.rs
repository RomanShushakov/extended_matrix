use std::ops::{Sub, Add, Mul, MulAssign, Div, Rem, SubAssign};
use std::any::Any;
use std::fmt::Debug;
use std::collections::{HashMap};
use std::hash::Hash;

use crate::basic_matrix::basic_matrix::BasicMatrixTrait;
use crate::basic_matrix::basic_matrix::BasicMatrixType;

use crate::basic_matrix::non_symmetric_matrix::NonSymmetricMatrix;

use crate::basic_matrix::functions::{matrix_size_check, copy_value_by_index};

use crate::shape::Shape;
use crate::matrix_element_position::MatrixElementPosition;


#[derive(Debug, Clone)]
pub struct SymmetricMatrix<T, V>
{
    rows_and_columns_number: T,
    elements_indexes: Vec<T>,
    elements_values: Vec<V>,
}


impl<T, V> BasicMatrixTrait<T, V> for SymmetricMatrix<T, V>
    where T: Copy + PartialOrd + Sub<Output = T> + Add<Output = T> + Mul<Output = T> + From<u8> +
             Div<Output = T> + Debug + Rem<Output = T> + Eq + Hash + SubAssign + 'static,
          V: Copy + Debug + PartialEq + MulAssign + From<f32> + 'static,
{
    fn read_element_value(&self, row: T, column: T) -> Result<V, &str>
    {
        matrix_size_check(
            row, column,
            (self.rows_and_columns_number, self.rows_and_columns_number))?;
        let (row, column) = if row <= column { (row, column) } else { (column, row) };
        let requested_index = row * self.rows_and_columns_number + column;
        let value = copy_value_by_index(
            requested_index, self.elements_indexes.as_slice(),
            self.elements_values.as_slice());
        Ok(value)
    }


    fn copy_all_elements_values(&self) -> HashMap<MatrixElementPosition<T>, V>
    {
        let mut all_elements_values = HashMap::new();
        for (index, value) in self.elements_indexes.iter()
            .zip(self.elements_values.iter())
        {
            let row = *index / self.rows_and_columns_number;
            let column = *index % self.rows_and_columns_number;
            let position = MatrixElementPosition::create(row, column);
            all_elements_values.insert(position, *value);
            if row != column
            {
                let symmetric_position = MatrixElementPosition::create(
                    column, row);
                all_elements_values.insert(symmetric_position, *value);
            }
        }
        all_elements_values
    }


    fn copy_shape(&self) -> Shape<T>
    {
        Shape(self.rows_and_columns_number, self.rows_and_columns_number)
    }


    fn transpose(&mut self) { }


    fn multiply_by_number(&mut self, number: V)
    {
        for i in 0..self.elements_values.len()
        {
            self.elements_values[i] *= number;
        }
    }


    fn into_symmetric(self) -> Box<dyn BasicMatrixTrait<T, V>>
    {
        Box::new(self)
    }



    fn define_type(&self) -> BasicMatrixType
    {
        BasicMatrixType::Symmetric
    }


    fn remove_zeros_rows_columns(&mut self) -> Vec<MatrixElementPosition<T>>
    {
        let mut zeros_rows_columns = Vec::new();
        let mut can_continue = true;
        while can_continue
        {
            if let Some(row_column) = self.find_zeros_row_column()
            {
                let zeros_row_column = MatrixElementPosition::create(
                    row_column, row_column);
                zeros_rows_columns.push(zeros_row_column);
                self.remove_row_column(row_column);
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
        let symmetric_matrix = self.clone();
        let mut non_symmetric_matrix = symmetric_matrix.non_symmetric();
        non_symmetric_matrix.remove_row(row);
        Box::new(non_symmetric_matrix)
    }


    fn remove_selected_column(&mut self, column: T) -> Box<dyn BasicMatrixTrait<T, V>>
    {
        let symmetric_matrix = self.clone();
        let mut non_symmetric_matrix = symmetric_matrix.non_symmetric();
        non_symmetric_matrix.remove_column(column);
        Box::new(non_symmetric_matrix)
    }


    fn as_any(&self) -> &dyn Any
    {
        self
    }
}


impl<T, V> SymmetricMatrix<T, V>
    where T: Copy + Debug + PartialEq + From<u8> + Mul<Output = T> + Add<Output = T> +
             PartialOrd + SubAssign + Div<Output = T> + Sub<Output = T> + Rem<Output = T>,
          V: Copy
{
    pub fn create(rows_and_columns_number: T, elements_indexes: Vec<T>, elements_values: Vec<V>)
        -> Self
    {
        SymmetricMatrix { rows_and_columns_number, elements_indexes, elements_values }
    }


    pub fn ref_rows_and_columns_number(&self) -> &T
    {
        &self.rows_and_columns_number
    }


    pub fn ref_elements_indexes(&self) -> &[T]
    {
        self.elements_indexes.as_slice()
    }


    pub fn ref_elements_values(&self) -> &[V]
    {
        self.elements_values.as_slice()
    }


    fn find_zeros_row_column(&self) -> Option<T>
    {
        let mut zeros_row_column = None;
        let find_index = |row, column| self.elements_indexes
            .iter()
            .position(|index|
                {
                    *index == row * self.rows_and_columns_number + column
                });

        let mut row_column = self.rows_and_columns_number;
        while row_column > T::from(0u8)
        {
            row_column -= T::from(1u8);
            let mut answers = Vec::new();
            let mut row = self.rows_and_columns_number;
            while row > T::from(0u8)
            {
                row -= T::from(1u8);
                match find_index(row, row_column)
                {
                    None => answers.push(true),
                    Some(_) => answers.push(false),
                }
            }
            let mut column = self.rows_and_columns_number;
            while column > T::from(0u8)
            {
                column -= T::from(1u8);
                match find_index(row_column, column)
                {
                    None => answers.push(true),
                    Some(_) => answers.push(false),
                }
            }
            if answers.iter().all(|answer| *answer == true)
            {
                zeros_row_column = Some(row_column);
            }
            if zeros_row_column != None
            {
                break;
            }
        }
        zeros_row_column
    }


    fn remove_row_column(&mut self, row_column: T)
    {
        for index in self.elements_indexes.as_mut_slice()
        {
            if *index >= row_column * self.rows_and_columns_number
            {
                *index -= self.rows_and_columns_number;
            }
        }
        for index in self.elements_indexes.as_mut_slice()
        {
            if *index % self.rows_and_columns_number > row_column
            {
                *index -= *index / self.rows_and_columns_number + T::from(1u8);
            }
            else
            {
                *index -= *index / self.rows_and_columns_number;
            }
        }
        self.rows_and_columns_number -= T::from(1u8);
    }


    fn non_symmetric(&self) -> NonSymmetricMatrix<T, V>
    {
        let non_symmetric_rows_number = self.rows_and_columns_number;
        let non_symmetric_columns_number = self.rows_and_columns_number;
        let mut non_symmetric_indexes = Vec::new();
        let mut non_symmetric_values = Vec::new();
        for (index, value) in self.elements_indexes.iter().zip(self.elements_values.iter())
        {
            non_symmetric_indexes.push(*index);
            non_symmetric_values.push(*value);
            let current_row = *index / non_symmetric_columns_number;
            let current_column = *index % non_symmetric_columns_number;
            if current_row != current_column
            {
                let symmetric_index = current_column * non_symmetric_columns_number + current_row;
                non_symmetric_indexes.push(symmetric_index);
                non_symmetric_values.push(*value);
            }
        }
        let non_symmetric_matrix = NonSymmetricMatrix::create(
            non_symmetric_rows_number, non_symmetric_columns_number,
            non_symmetric_indexes, non_symmetric_values);
        non_symmetric_matrix
    }
}
