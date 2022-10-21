use std::fmt::Debug;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::enums::Operation;
use crate::traits::{UIntTrait, FloatTrait};

use crate::shape::Shape;
use crate::matrix_element_position::MatrixElementPosition;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BasicMatrixType
{
    Symmetric,
    NonSymmetric
}


#[derive(Clone, Debug)]
pub struct BasicMatrix<T, V>
{
    rows_number: T,
    columns_number: T,
    matrix_type: BasicMatrixType,
    elements_values: HashMap<MatrixElementPosition<T>, V>
}


impl<T, V> BasicMatrix<T, V>
    where T: UIntTrait<Output = T>,
          V: FloatTrait<Output = V, Other = V>
{
    pub fn create_default(rows_number: T, columns_number: T, matrix_type: BasicMatrixType) -> Self
    {
        BasicMatrix { rows_number, columns_number, matrix_type, elements_values: HashMap::new() }
    }


    pub fn create(rows_number: T, columns_number: T, all_elements_values: Vec<V>, tolerance: V)
        -> Result<Self, String>
    {
        let mut index = 0usize;
        let mut row = T::from(0u8);
        let mut symmetric_elements_values = HashMap::new();
        let mut is_symmetric = if rows_number == columns_number { true } else { false };
        let mut nonsymmetric_elements_values = HashMap::new();
        while row < rows_number
        {
            let mut column = T::from(0u8);
            while column < columns_number
            {
                if index >= all_elements_values.len()
                {
                    return Err("Basic matrix: Incorrect number of elements!".to_string());
                }
                let mut matrix_element_position =
                    MatrixElementPosition::create(row, column);
                let element_value = all_elements_values[index];

                if element_value.into().abs() > tolerance.into()
                {
                    nonsymmetric_elements_values.insert(matrix_element_position.clone(),
                        element_value);
                }

                if row <= column && is_symmetric
                {
                    if element_value.into().abs() > tolerance.into()
                    {
                        symmetric_elements_values.insert(matrix_element_position.clone(),
                            element_value);
                    }
                }

                if row > column
                {
                    matrix_element_position.swap_row_and_column();
                    if let Some(symmetric_element_value) = symmetric_elements_values
                        .get(&matrix_element_position)
                    {
                        if *symmetric_element_value != element_value
                        {
                            is_symmetric = false;
                        }
                    }
                    else
                    {
                        if element_value.into().abs() > tolerance.into()
                        {
                            is_symmetric = false;
                        }
                    }
                }
                column += T::from(1u8);
                index += 1usize;
            }
            row += T::from(1u8);
        }

        let (matrix_type, elements_values) =
            {
                if is_symmetric
                {
                    (BasicMatrixType::Symmetric, symmetric_elements_values)
                }
                else
                {
                    (BasicMatrixType::NonSymmetric, nonsymmetric_elements_values)
                }
            };

        Ok(BasicMatrix { rows_number, columns_number, matrix_type, elements_values })
    }


    fn matrix_size_check(&self, ref_matrix_element_position: &MatrixElementPosition<T>)
        -> Result<(), String>
    {
        let ref_row = ref_matrix_element_position.ref_row();
        let ref_column = ref_matrix_element_position.ref_column();

        if *ref_row >= self.rows_number || *ref_column >= self.columns_number
        {
            return Err("Basic matrix: Inputted indexes are out of matrix size!".to_string());
        }
        Ok(())
    }


    pub fn insert_matrix_element(&mut self, matrix_element_position: MatrixElementPosition<T>,
        element_value: V, tolerance: V)
    {
        if element_value.into().abs() > tolerance.into()
        {
            match self.matrix_type
            {
                BasicMatrixType::Symmetric =>
                    {
                        let ref_row = matrix_element_position.ref_row();
                        let ref_column = matrix_element_position.ref_column();
                        if ref_row <= ref_column
                        {
                            self.elements_values.insert(matrix_element_position,
                                element_value);
                        }
                    },
                BasicMatrixType::NonSymmetric =>
                    {
                        self.elements_values.insert(matrix_element_position, element_value);
                    },
            }
        }
    }


    pub fn remove_matrix_element(&mut self, matrix_element_position: MatrixElementPosition<T>)
    {
        let _ = self.elements_values.remove(&matrix_element_position);
    }


    pub fn add_sub_mul_assign_matrix_element_value(&mut self,
        matrix_element_position: MatrixElementPosition<T>, element_value: V, operation: Operation)
    {
        let handler = |elements_values: &mut HashMap<MatrixElementPosition<T>, V>|
            {
                if let Some(existed_element_value) =
                    elements_values.get_mut(&matrix_element_position)
                {
                    match operation
                    {
                        Operation::Addition => *existed_element_value += element_value,
                        Operation::Subtraction => *existed_element_value -= element_value,
                        Operation::Multiplication => *existed_element_value *= element_value,
                    }
                }
                else
                {
                    match operation
                    {
                        Operation::Addition =>
                            {
                                elements_values.insert(matrix_element_position.clone(),
                                    element_value);
                            },
                        Operation::Subtraction =>
                            {
                                elements_values.insert(matrix_element_position.clone(),
                                    element_value * V::from(-1f32));
                            },
                        Operation::Multiplication => ()
                    }
                }
            };

        match &self.matrix_type
        {
            BasicMatrixType::Symmetric =>
                {
                    let ref_row = matrix_element_position.ref_row();
                    let ref_column = matrix_element_position.ref_column();
                    if ref_row <= ref_column
                    {
                        handler(&mut self.elements_values);
                    }
                },
            BasicMatrixType::NonSymmetric =>
                {
                    handler(&mut self.elements_values);
                },
        }
    }


    fn remove_nonzero_values_from_row(&self, row: T, zero_columns: &mut HashSet<T>)
    {
        for column in zero_columns.clone().into_iter()
        {
            let matrix_element_position =
                MatrixElementPosition::create(row, column);
            if self.elements_values.contains_key(&matrix_element_position)
            {
                zero_columns.remove(&column);
            }
        }
    }


    fn remove_nonzero_values_from_column(&self, column: T, zero_rows: &mut HashSet<T>)
    {
        for row in zero_rows.clone().into_iter()
        {
            let matrix_element_position =
                MatrixElementPosition::create(row, column);
            if self.elements_values.contains_key(&matrix_element_position)
            {
                zero_rows.remove(&row);
            }
        }
    }


    pub fn remove_selected_row(&mut self, row: T)
    {
        self.into_nonsymmetric();

        let mut column = self.columns_number;
        while column > T::from(0u8)
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
            if *matrix_element_position.ref_row() > row
            {
                matrix_element_position.decrease_row_number();
            }
            updated_elements_values.insert(matrix_element_position, element);
        }
        self.rows_number -= T::from(1u8);
        self.elements_values = updated_elements_values;
    }


    pub fn remove_selected_column(&mut self, column: T)
    {
        self.into_nonsymmetric();

        let mut row = self.rows_number;
        while row > T::from(0u8)
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
            if *matrix_element_position.ref_column() > column
            {
                matrix_element_position.decrease_column_number();
            }
            updated_elements_values.insert(matrix_element_position, element);
        }
        self.columns_number -= T::from(1u8);
        self.elements_values = updated_elements_values;
        self.into_nonsymmetric();
    }


    pub fn copy_element_value_or_zero(&self, mut matrix_element_position: MatrixElementPosition<T>)
        -> Result<V, String>
    {
        self.matrix_size_check(&matrix_element_position)?;
        if self.matrix_type == BasicMatrixType::Symmetric
        {
            if matrix_element_position.ref_row() > matrix_element_position.ref_column()
            {
                matrix_element_position.swap_row_and_column();
            }
        }

        let element_value =
            if let Some(value) = self.elements_values.get(&matrix_element_position)
            {
                *value
            }
            else { V::from(0f32) };
        Ok(element_value)
    }


    pub fn transpose(&mut self)
    {
        match self.ref_matrix_type()
        {
            BasicMatrixType::Symmetric => (),
            BasicMatrixType::NonSymmetric => 
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
        }
    }


    pub fn multiply_by_number(&mut self, number: V)
    {
        for element_value in self.elements_values.values_mut()
        {
            *element_value *= number;
        }
    }


    pub fn remove_zeros_rows_columns(&mut self) -> Vec<MatrixElementPosition<T>>
    {
        let mut zero_rows = HashSet::new();
        let mut zero_row = T::from(0u8);
        while zero_row < self.rows_number
        {
            zero_rows.insert(zero_row);
            zero_row += T::from(1u8);
        }

        let mut zero_columns = HashSet::new();
        let mut zero_column = T::from(0u8);
        while zero_column < self.columns_number
        {
            zero_columns.insert(zero_column);
            zero_column += T::from(1u8);
        }

        let mut row = T::from(0u8);
        while row < self.rows_number
        {
            let mut column = T::from(0u8);
            while column < self.columns_number
            {
                self.remove_nonzero_values_from_row(row, &mut zero_columns);
                self.remove_nonzero_values_from_column(column, &mut zero_rows);

                column += T::from(1u8);
            }
            row += T::from(1u8);
        }

        let mut rows_for_remove = zero_rows.into_iter().collect::<Vec<T>>();
        rows_for_remove.sort();

        let mut columns_for_remove = zero_columns.into_iter().collect::<Vec<T>>();
        columns_for_remove.sort();

        let mut zeros_rows_columns= Vec::new();
        for (row, column) in rows_for_remove.into_iter().rev().zip(columns_for_remove.into_iter().rev())
        {
            let matrix_element_position =
                MatrixElementPosition::create(row, column);
            zeros_rows_columns.push(matrix_element_position);
            self.remove_selected_row(row);
            self.remove_selected_column(column);
        }

        zeros_rows_columns
    }


    pub fn try_to_symmetrize(&mut self, tolerance: V)
    {
        if self.matrix_type == BasicMatrixType::NonSymmetric
        {
            let mut symmetric_elements_values = HashMap::new();
            for (matrix_element_position, element_value) in
                self.elements_values.iter()
            {
                let ref_row = matrix_element_position.ref_row();
                let ref_column = matrix_element_position.ref_column();
                if ref_row == ref_column
                {
                    symmetric_elements_values.insert(matrix_element_position.clone(),
                        *element_value);
                }
                else
                {
                    let symmetric_matrix_element_position =
                        MatrixElementPosition::create(*ref_column, *ref_row);
                    if let Some(symmetric_element_value) =
                        self.elements_values.get(&symmetric_matrix_element_position)
                    {
                        if (*element_value - *symmetric_element_value).my_abs() < tolerance
                        {
                            let (symmetric_row, symmetric_column) =
                                {
                                    if ref_row > ref_column
                                    {
                                        (*ref_column, *ref_row)
                                    }
                                    else
                                    {
                                        (*ref_row, *ref_column)
                                    }
                                };
                            let symmetric_matrix_element_position =
                                MatrixElementPosition::create(symmetric_row, symmetric_column);
                            symmetric_elements_values.insert(symmetric_matrix_element_position,
                                *symmetric_element_value);
                        }
                        else
                        {
                            return;
                        }
                    }
                    else
                    {
                        return;
                    }
                }
            }
            self.elements_values = symmetric_elements_values;
            self.matrix_type = BasicMatrixType::Symmetric;
        }
    }


    pub fn into_nonsymmetric(&mut self)
    {

        if self.matrix_type == BasicMatrixType::Symmetric
        {
            let mut symmetric_elements_values = HashMap::new();
            for (matrix_element_position, element_value) in
                self.elements_values.iter()
            {
                let ref_row = matrix_element_position.ref_row();
                let ref_column = matrix_element_position.ref_column();
                if ref_row != ref_column
                {
                    let symmetric_matrix_element_position =
                        MatrixElementPosition::create(*ref_column, *ref_row);
                    symmetric_elements_values.insert(symmetric_matrix_element_position,
                        *element_value);
                }
            }
            self.elements_values.extend(symmetric_elements_values);
            self.matrix_type = BasicMatrixType::NonSymmetric;
        }
    }


    pub fn copy_shape(&self) -> Shape<T>
    {
        Shape(self.rows_number, self.columns_number)
    }


    pub fn ref_matrix_type(&self) -> &BasicMatrixType
    {
        &self.matrix_type
    }


    pub fn ref_elements_values(&self) -> &HashMap<MatrixElementPosition<T>, V>
    {
        &self.elements_values
    }


    // pub fn clone_all_elements_values(&self) -> HashMap<MatrixElementPosition<T>, V>
    // {
    //     match self.matrix_type
    //     {
    //         BasicMatrixType::NonSymmetric => self.elements_values.clone(),
    //         BasicMatrixType::Symmetric =>
    //             {
    //                 let mut basic_matrix = self.clone();
    //                 basic_matrix.into_nonsymmetric();
    //                 basic_matrix.elements_values
    //             }
    //     }
    // }
}


impl<T, V> PartialEq for BasicMatrix<T, V>
    where T: PartialEq + Eq + Hash,
          V: PartialEq
{
    fn eq(&self, other: &Self) -> bool 
    {
        if self.rows_number != other.rows_number || self.columns_number != other.columns_number || 
            self.matrix_type != other.matrix_type
        {
            return false;
        }

        for (matrix_element_position, element_value) in self.elements_values.iter()
        {
            match other.elements_values.get(matrix_element_position)
            {
                None => return false,
                Some(v) => 
                {
                    if v != element_value
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}
