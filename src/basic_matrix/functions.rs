use std::fmt::Debug;
use std::ops::{Mul, Add, SubAssign, Div, Sub, Rem};

use crate::one::One;

use crate::basic_matrix::basic_matrix::BasicMatrixTrait;

use crate::basic_matrix::symmetric_matrix::SymmetricMatrix;

use crate::basic_matrix::non_symmetric_matrix::NonSymmetricMatrix;



pub fn matrix_size_check<'a, T>(inputted_row: T, inputted_column: T, matrix_size: (T, T))
    -> Result<(), &'a str>
    where T: PartialOrd
{
    if inputted_row >= matrix_size.0 || inputted_column >= matrix_size.1
    {
        return Err("Basic matrix: Inputted indexes are out of matrix size!");
    }
    Ok(())
}


pub fn extract_value_by_index<T, V>(requested_index: T, indexes: &[T], values: &[V]) -> V
    where T: Copy + PartialEq,
          V: Copy + Default
{
    if let Some(position) = indexes
        .iter().position(|index| *index == requested_index)
    {
        values[position]
    }
    else
    {
        V::default()
    }
}


pub fn return_symmetric_matrix_struct<T, V>(boxed_struct: Box<dyn BasicMatrixTrait<T, V>>)
    -> SymmetricMatrix<T, V>
    where T: Copy + Debug + PartialEq + One + Default + Mul<Output = T> + Add<Output = T> +
             PartialOrd + SubAssign + Div<Output = T> + Sub<Output = T> + Rem<Output = T> + 'static,
          V: Copy + Default + 'static
{
    let matrix: &SymmetricMatrix<T, V> = match boxed_struct
        .as_any()
        .downcast_ref::<SymmetricMatrix<T, V>>()
        {
            Some(matrix) => matrix,
            None => panic!("Basic matrix: Matrix is not symmetric!!!"),
        };
    let rows_and_columns_number = matrix.rows_and_columns_number();
    let elements_indexes = matrix.elements_indexes();
    let elements_values = matrix.elements_values();
    let symmetric_matrix = SymmetricMatrix::create(rows_and_columns_number,
        elements_indexes, elements_values);
    symmetric_matrix
}


pub fn return_non_symmetric_matrix_struct<T, V>(boxed_struct: Box<dyn BasicMatrixTrait<T, V>>)
    -> NonSymmetricMatrix<T, V>
    where T: Copy + Debug + PartialEq + Default + One + Mul<Output = T> + Add<Output = T> +
             PartialOrd + SubAssign + Div<Output = T> + Sub<Output = T> + Rem<Output = T> + 'static,
          V: Copy + Debug + Default + 'static
{
    let matrix: &NonSymmetricMatrix<T, V> = match boxed_struct
        .as_any()
        .downcast_ref::<NonSymmetricMatrix<T, V>>()
        {
            Some(matrix) => matrix,
            None => panic!("Basic matrix: Matrix is symmetric!!!"),
        };
    let rows_number = matrix.rows_number();
    let columns_number = matrix.columns_number();
    let elements_indexes = matrix.elements_indexes();
    let elements_values = matrix.elements_values();

    let non_symmetric_matrix = NonSymmetricMatrix::create(rows_number,
        columns_number, elements_indexes, elements_values);

    non_symmetric_matrix
}
