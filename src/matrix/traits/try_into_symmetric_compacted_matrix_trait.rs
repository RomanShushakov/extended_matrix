use extended_matrix_float::MyFloatTrait;

use crate::{BasicOperationsTrait, SquareMatrixTrait, Position};
use crate::FloatTrait;


pub trait TryIntoSymmetricCompactedMatrixTrait: SquareMatrixTrait
{
    fn try_into_symmetric_compacted_matrix(&self, rel_tol: <Self as BasicOperationsTrait>::Value) 
        -> Result<(Vec<<Self as BasicOperationsTrait>::Value>, Vec<i64>), String>
        where <Self as BasicOperationsTrait>::Value: FloatTrait<Output = <Self as BasicOperationsTrait>::Value>
    {
        let shape = self.get_shape();
        let mut a = Vec::new();
        let mut maxa = Vec::new();
        let mut index = 0i64;

        for column in 0..shape.1
        {
            if *self.get_element_value(&Position(column, column)).expect("Element is absent") == 
                <<Self as BasicOperationsTrait>::Value>::from(0f32)
            {
                return Err(format!("Diagonal element [{}, {}] equals to zero!", column, column));
            }
            let mut skyline = 0;
            'skyline: while skyline < column
            {
                if *self.get_element_value(&Position(skyline, column)).expect("Element is absent") != 
                    <<Self as BasicOperationsTrait>::Value>::from(0f32)
                {
                    break 'skyline;
                }
                skyline += 1;
            }
            let mut row = column;
            maxa.push(index);
            index += 1;
            if row > 0
            {
                while row > skyline
                {
                    let value = 
                        *self.get_element_value(&Position(row, column)).expect("Element is absent");
                    let symm_value = 
                        *self.get_element_value(&Position(column, row)).expect("Element is absent");
                    if (<<Self as BasicOperationsTrait>::Value>::from(1f32) - value / symm_value).my_abs() > rel_tol
                    {
                        return Err(format!("Element [{row}, {column}] does not match with \
                            [{column}, {row}]!"));
                    }
                    a.push(value);
                    row -= 1;
                    if row != column
                    {
                        index += 1;
                    }
                }
                let value = 
                    *self.get_element_value(&Position(row, column)).expect("Element is absent");
                let symm_value = 
                    *self.get_element_value(&Position(column, row)).expect("Element is absent");
                if (<<Self as BasicOperationsTrait>::Value>::from(1f32) - value / symm_value).my_abs() > rel_tol
                {
                    return Err(format!("Element [{row}, {column}] does not match with \
                        [{column}, {row}]!"));
                }
                a.push(value);
            }
            else
            {
                let value = 
                    *self.get_element_value(&Position(row, column)).expect("Element is absent");
                let symm_value = 
                    *self.get_element_value(&Position(column, row)).expect("Element is absent");
                if (<<Self as BasicOperationsTrait>::Value>::from(1f32) - value / symm_value).my_abs() > rel_tol
                {
                    return Err(format!("Element [{row}, {column}] does not match with \
                        [{column}, {row}]!"));
                }
                a.push(value);
            }
        }
        maxa.push(index);

        Ok((a, maxa))
    }


    fn forced_into_symmetric_compacted_matrix(&self, rel_tol: <Self as BasicOperationsTrait>::Value, 
        warnings: &mut Vec<String>) -> (Vec<<Self as BasicOperationsTrait>::Value>, Vec<i64>)
        where <Self as BasicOperationsTrait>::Value: FloatTrait<Output = <Self as BasicOperationsTrait>::Value>
    {
        let shape = self.get_shape();
        let mut a = Vec::new();
        let mut maxa = Vec::new();
        let mut index = 0i64;

        for column in 0..shape.1
        {
            if *self.get_element_value(&Position(column, column)).expect("Element is absent") == 
                <<Self as BasicOperationsTrait>::Value>::from(0f32)
            {
                let warning = format!("Diagonal element [{}, {}] equals to zero!", column, column);
                warnings.push(warning);
            }
            let mut skyline = 0;
            'skyline: while skyline < column
            {
                if *self.get_element_value(&Position(skyline, column)).expect("Element is absent") != 
                    <<Self as BasicOperationsTrait>::Value>::from(0f32)
                {
                    break 'skyline;
                }
                skyline += 1;
            }
            let mut row = column;
            maxa.push(index);
            index += 1;
            if row > 0
            {
                while row > skyline
                {
                    let value = 
                        *self.get_element_value(&Position(row, column)).expect("Element is absent");
                    let symm_value = 
                        *self.get_element_value(&Position(column, row)).expect("Element is absent");
                    if (<<Self as BasicOperationsTrait>::Value>::from(1f32) - value / symm_value).my_abs() > rel_tol
                    {
                        let warning = format!("Element [{row}, {column}] does not match with \
                            [{column}, {row}]!");
                        warnings.push(warning);
                    }
                    a.push(value);
                    row -= 1;
                    if row != column
                    {
                        index += 1;
                    }
                }
                let value = 
                    *self.get_element_value(&Position(row, column)).expect("Element is absent");
                let symm_value = 
                    *self.get_element_value(&Position(column, row)).expect("Element is absent");
                if (<<Self as BasicOperationsTrait>::Value>::from(1f32) - value / symm_value).my_abs() > rel_tol
                {
                    let warning = format!("Element [{row}, {column}] does not match with \
                        [{column}, {row}]!");
                    warnings.push(warning);
                }
                a.push(value);
            }
            else
            {
                let value = 
                    *self.get_element_value(&Position(row, column)).expect("Element is absent");
                let symm_value = 
                    *self.get_element_value(&Position(column, row)).expect("Element is absent");
                if (<<Self as BasicOperationsTrait>::Value>::from(1f32) - value / symm_value).my_abs() > rel_tol
                {
                    let warning = format!("Element [{row}, {column}] does not match with \
                        [{column}, {row}]!");
                    warnings.push(warning);
                }
                a.push(value);
            }
        }
        maxa.push(index);

        (a, maxa)
    }
}
