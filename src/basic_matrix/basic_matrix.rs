use std::fmt::Debug;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::ops::{Mul, Add, Sub, Div, Rem, MulAssign, SubAssign, AddAssign};
use std::hash::Hash;

use crate::basic_matrix::functions::matrix_size_check;
use crate::functions::conversion_uint_into_usize;


#[derive(Debug, Copy, Clone)]
pub enum BasicMatrixType
{
    Symmetric,
    NonSymmetric
}


#[derive(PartialEq)]
pub struct Shape<T>(pub T, pub T);


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct MatrixElementPosition<T>
{
    row: T,
    column: T,
}


impl<T> MatrixElementPosition<T>
    where T: Copy + From<u8> + SubAssign
{
    pub fn create(row: T, column: T) -> Self
    {
        MatrixElementPosition { row, column }
    }


    pub fn row(&self) -> T
    {
        self.row
    }


    pub fn column(&self) -> T
    {
        self.column
    }


    pub fn swap_row_and_column(&mut self)
    {
        let interim = self.row;
        self.row = self.column;
        self.column = interim;
    }


    pub fn decrease_row_number(&mut self)
    {
        self.row -= T::from(1u8);
    }


    pub fn decrease_column_number(&mut self)
    {
        self.column -= T::from(1u8);
    }
}


pub trait BasicMatrixClone<T, V>
{
    fn clone_box(&self) -> Box<dyn BasicMatrixTrait<T, V>>;
}


impl<T, V, W> BasicMatrixClone<T, V> for W
    where W: BasicMatrixTrait<T, V> + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn BasicMatrixTrait<T, V>>
    {
        Box::new(self.clone())
    }
}


impl<T, V> Clone for Box<dyn BasicMatrixTrait<T, V>>
{
    fn clone(&self) -> Box<dyn BasicMatrixTrait<T, V>>
    {
        self.clone_box()
    }
}


pub trait BasicMatrixTrait<T, V>: BasicMatrixClone<T, V>
{
    fn read_element_value(&self, row: T, column: T) -> Result<V, &str>;
    fn copy_all_elements_values(&self) -> HashMap<MatrixElementPosition<T>, V>;
    fn copy_shape(&self) -> Shape<T>;
    fn transpose(&mut self);
    fn multiply_by_number(&mut self, number: V);
    fn into_symmetric(self) -> Box<dyn BasicMatrixTrait<T, V>>;
    fn define_type(&self) -> BasicMatrixType;
    fn remove_zeros_rows_columns(&mut self) -> Vec<MatrixElementPosition<T>>;
    fn remove_selected_row(&mut self, row: T) -> Box<dyn BasicMatrixTrait<T, V>>;
    fn remove_selected_column(&mut self, column: T) -> Box<dyn BasicMatrixTrait<T, V>>;
    fn as_any(&self) -> &dyn Any;
}


#[derive(Clone)]
pub struct BasicMatrix<T, V>
{
    rows_number: T,
    columns_number: T,
    matrix_type: BasicMatrixType,
    elements_values: HashMap<MatrixElementPosition<T>, V>
}


impl<T, V> BasicMatrix<T, V>
    where T: Copy + PartialEq + Debug + PartialOrd + Mul<Output = T> + Add<Output = T> +
                 Sub<Output = T> + Div<Output = T> + Rem<Output = T> + Eq + Hash + SubAssign +
                 AddAssign + From<u8> + 'static,
              V: Copy + PartialEq + Debug + MulAssign + From<f32> + 'static,
{
    fn is_row_of_zeros(&self, row: T, nonzero_columns: &mut HashSet<T>) -> bool
    {
        let mut column = T::from(0u8);
        while column < self.columns_number
        {
            if nonzero_columns.contains(&column)
            {
                return false;
            }
            let matrix_element_position =
                MatrixElementPosition::create(row, column);
            if self.elements_values.contains_key(&matrix_element_position)
            {
                nonzero_columns.insert(column);
                return false;
            }
            column += T::from(1u8);
        }
        true
    }


    fn is_column_of_zeros(&self, column: T, nonzero_rows: &mut HashSet<T>) -> bool
    {
        let mut row = T::from(0u8);
        while row < self.rows_number
        {
            if nonzero_rows.contains(&row)
            {
                return false;
            }
            let matrix_element_position =
                MatrixElementPosition::create(row, column);
            if self.elements_values.contains_key(&matrix_element_position)
            {
                nonzero_rows.insert(row);
                return false;
            }
            row += T::from(1u8);
        }
        true
    }


    fn remove_row(&mut self, row: T)
    {
        self.matrix_type = BasicMatrixType::NonSymmetric;
        let mut column = self.columns_number;
        while column > T::from(1u8)
        {
            column -= T::from(1u8);
            let matrix_element_position =
                MatrixElementPosition::create(row, column);
            let _ = self.elements_values.remove(&matrix_element_position);
        }
        let mut updated_elements_values = HashMap::new();
        for (mut matrix_element_position, element) in
            self.elements_values.clone().into_iter()
        {
            if matrix_element_position.row() > row
            {
                matrix_element_position.decrease_row_number();
            }
            updated_elements_values.insert(matrix_element_position, element);
        }
        self.rows_number -= T::from(1u8);
        self.elements_values = updated_elements_values;
    }


    fn remove_column(&mut self, column: T)
    {
        self.matrix_type = BasicMatrixType::NonSymmetric;
        let mut row = self.rows_number;
        while row > T::from(1u8)
        {
            row -= T::from(1u8);
            let matrix_element_position =
                MatrixElementPosition::create(row, column);
            let _ = self.elements_values.remove(&matrix_element_position);
        }
        let mut updated_elements_values = HashMap::new();
        for (mut matrix_element_position, element) in
            self.elements_values.clone().into_iter()
        {
            if matrix_element_position.column() > column
            {
                matrix_element_position.decrease_column_number();
            }
            updated_elements_values.insert(matrix_element_position, element);
        }
        self.columns_number -= T::from(1u8);
        self.elements_values = updated_elements_values;
    }
}


impl<T, V> BasicMatrixTrait<T, V> for BasicMatrix<T, V>
    where T: Copy + PartialEq + Debug + PartialOrd + Mul<Output = T> + Add<Output = T> +
             Sub<Output = T> + Div<Output = T> + Rem<Output = T> + Eq + Hash + SubAssign +
             AddAssign + From<u8> + 'static,
          V: Copy + PartialEq + Debug + MulAssign + From<f32> + 'static,
{
    fn read_element_value(&self, row: T, column: T) -> Result<V, &str>
    {
        matrix_size_check(row, column,
            (self.rows_number, self.columns_number))?;
        let matrix_element_position =
            MatrixElementPosition::create(row, column);
        if let Some(value) = self.elements_values.get(&matrix_element_position)
        {
            Ok(*value)
        }
        else
        {
            Ok(V::from(0f32))
        }
    }


    fn copy_all_elements_values(&self) -> HashMap<MatrixElementPosition<T>, V>
    {
        self.elements_values.clone()
    }


    fn copy_shape(&self) -> Shape<T>
    {
        Shape(self.rows_number, self.columns_number)
    }


    fn transpose(&mut self)
    {
        let transposed_rows_number = self.columns_number;
        let transposed_columns_number = self.rows_number;
        let mut transposed_elements = HashMap::new();
        for (mut matrix_element_position, element) in
            self.elements_values.clone().into_iter()
        {
            matrix_element_position.swap_row_and_column();
            transposed_elements.insert(matrix_element_position, element);
        }
        self.elements_values = transposed_elements;
        self.rows_number = transposed_rows_number;
        self.columns_number = transposed_columns_number;
    }


    fn multiply_by_number(&mut self, number: V)
    {
        for element_value in self.elements_values.values_mut()
        {
            *element_value *= number;
        }
    }


    fn into_symmetric(self) -> Box<dyn BasicMatrixTrait<T, V>>
    {
        return Box::new(self);
    }


    fn define_type(&self) -> BasicMatrixType
    {
        self.matrix_type
    }


    fn remove_zeros_rows_columns(&mut self) -> Vec<MatrixElementPosition<T>>
    {
        let mut zeros_rows_columns = Vec::new();
        let mut nonzero_rows = HashSet::new();
        let mut nonzero_columns = HashSet::new();
        let mut row = T::from(0u8);

        'outer: while row < self.rows_number
        {
            let mut column = T::from(0u8);
            'inner: while column < self.columns_number
            {
                if nonzero_rows.len() == conversion_uint_into_usize(self.rows_number) ||
                    nonzero_columns.len() == conversion_uint_into_usize(self.columns_number)
                {
                    break 'outer;
                }
                if self.is_row_of_zeros(row, &mut nonzero_columns) &&
                    self.is_column_of_zeros(column, &mut nonzero_rows)
                {
                    let matrix_element_position =
                        MatrixElementPosition::create(row, column);
                    zeros_rows_columns.push(matrix_element_position);
                }
                column += T::from(1u8);
            }
            row += T::from(1u8);
        }
        for matrix_element_position in zeros_rows_columns.iter().rev()
        {
            let row = matrix_element_position.row();
            let column = matrix_element_position.column();
            self.remove_row(row);
            self.remove_column(column);
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