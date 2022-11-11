use std::collections::HashMap;

use extended_matrix_float::MyFloatTrait;

use crate::matrix::{SymmetricMatrix, BasicOperationsTrait, SquareMatrixTrait, Position};
use crate::FloatTrait;


pub trait TryIntoSymmetricMatrixTrait: SquareMatrixTrait
{
    fn try_into_symmetric_matrix(&self, rel_tol: <Self as BasicOperationsTrait>::Value) 
        -> Result<SymmetricMatrix<<Self as BasicOperationsTrait>::Value>, String>
        where <Self as BasicOperationsTrait>::Value: FloatTrait<Output = <Self as BasicOperationsTrait>::Value>
    {
        let shape = self.get_shape();
        let mut elements = HashMap::new();

        for row_number in 0..shape.0
        {
            for column_number in 0..shape.1
            {
                let position = Position(row_number, column_number);
                let mut value = 
                    self.get_element_value(&position).expect("Element is absent");
                if row_number > column_number
                {
                    let mut symm_position = position.clone();
                    symm_position.swap_row_and_column();
                    let symm_value = 
                        self.get_element_value(&symm_position).expect("Element is absent");
                    if (<<Self as BasicOperationsTrait>::Value>::from(1f32) - *value / *symm_value).my_abs() > rel_tol
                    {
                        return Err(format!("Element [{row_number}, {column_number}] does not match with \
                            [{column_number}, {row_number}]!"));
                    }
                    value = symm_value;
                }
                elements.insert(position, *value);
            }
        }

        Ok(SymmetricMatrix { shape: shape.clone(), elements })
    }


    fn forced_into_symmetric_matrix(&self, rel_tol: <Self as BasicOperationsTrait>::Value, warnings: &mut Vec<String>) 
        -> SymmetricMatrix<<Self as BasicOperationsTrait>::Value>
        where <Self as BasicOperationsTrait>::Value: FloatTrait<Output = <Self as BasicOperationsTrait>::Value>
    {
        let shape = self.get_shape();
        let mut elements = HashMap::new();

        for row_number in 0..shape.0
        {
            for column_number in 0..shape.1
            {
                let position = Position(row_number, column_number);
                let mut value = 
                    self.get_element_value(&position).expect("Element is absent");
                if row_number > column_number
                {
                    let mut symm_position = position.clone();
                    symm_position.swap_row_and_column();
                    let symm_value = 
                        self.get_element_value(&symm_position).expect("Element is absent");
                    if (<<Self as BasicOperationsTrait>::Value>::from(1f32) - *value / *symm_value).my_abs() > rel_tol
                    {
                        let warning = format!("Element [{row_number}, {column_number}] does not match with \
                            [{column_number}, {row_number}]!");
                        warnings.push(warning);
                    }
                    value = symm_value;
                }
                elements.insert(position, *value);
            }
        }

        SymmetricMatrix { shape: shape.clone(), elements }
    }
}
