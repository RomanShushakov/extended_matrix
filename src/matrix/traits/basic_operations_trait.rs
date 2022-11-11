use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{AddAssign, SubAssign, Mul, MulAssign};

use crate::matrix::{NewShape, Position, Matrix, IntoMatrixTrait};
use crate::enums::Operation;


pub trait BasicOperationsTrait 
{
    type Value;

    fn get_shape(&self) -> &NewShape;
    fn get_mut_shape(&mut self) -> &mut NewShape;
    fn get_elements(&self) -> &HashMap<Position, Self::Value>;
    fn get_mut_elements(&mut self) -> &mut HashMap<Position, Self::Value>;


    fn get_element_value(&self, position: &Position) -> Result<&Self::Value, String>
        where Self::Value: Copy
    {
        self.get_elements().get(position).ok_or("Element is absent".to_string())
    }


    fn get_mut_element_value(&mut self, position: &Position) -> Result<&mut Self::Value, String>
        where Self::Value: Copy
    {
        self.get_mut_elements().get_mut(position).ok_or("Element is absent".to_string())
    }


    fn shape_conformity_check<M>(&self, other: &M, operation: Operation) -> Result<(), String>
        where M: BasicOperationsTrait
    {
        match operation 
        {
            Operation::Addition | Operation::Subtraction => 
            {
                if self.get_shape() != other.get_shape()
                {
                    return Err("Shapes of matrices do not conform to each other!".to_string());
                }
            },
            Operation::Multiplication => 
            {
                if self.get_shape().1 != other.get_shape().0
                {
                    return Err("Shapes of matrices do not conform to each other!".to_string());
                }
            }
        }
        Ok(())
    }


    fn add<M>(&self, other: &M) -> Result<Self, String>
        where M: BasicOperationsTrait<Value = Self::Value>, 
              Self::Value: Copy + AddAssign,
              Self: Clone,
    {
        self.shape_conformity_check::<M>(&other, Operation::Addition)?;
        let mut result = self.clone();
        for (position, value) in other.get_elements()
        {
            *result.get_mut_element_value(position).expect("Element is absent") += *value;
        }
        Ok(result)
    }


    fn subtract<M>(&self, other: &M) -> Result<Self, String>
        where M: BasicOperationsTrait<Value = Self::Value>, 
              Self::Value: Copy + SubAssign,
              Self: Clone,
    {
        self.shape_conformity_check::<M>(&other, Operation::Subtraction)?;
        let mut result = self.clone();
        for (position, value) in other.get_elements()
        {
            *result.get_mut_element_value(position).expect("Element is absent") -= *value;
        }
        Ok(result)
    }


    fn multiply_by_scalar(&self, scalar: Self::Value) -> Self
        where Self::Value: Copy + MulAssign,
              Self: Clone,
    {
        let mut result = self.clone();
        for value in result.get_mut_elements().values_mut()
        {
            *value *= scalar;
        }
        result
    }


    fn multiply<M>(&self, other: &M) -> Result<Matrix<Self::Value>, String>
        where M: IntoMatrixTrait<Value = Self::Value> + Clone,
              Self::Value: Copy + AddAssign + SubAssign + Mul<Output = Self::Value> + From<f32>,
    {
        self.shape_conformity_check::<M>(&other, Operation::Multiplication)?;
        let mut result = other.clone();
        let (rows_number, columns_number) = (self.get_shape().0, other.get_shape().1);
        result.get_mut_shape().update(rows_number, columns_number);
        result.get_mut_elements().clear();

        for i in 0..(rows_number * columns_number)
        {
            let mut result_value = <Self::Value>::from(0f32);
            for k in 0..self.get_shape().1
            {
                let self_position = Position(i / columns_number, k);
                let self_value = 
                    self.get_element_value(&self_position).expect("Element is absent");
                let other_position = Position(k, i % columns_number);
                let other_value = 
                    other.get_element_value(&other_position).expect("Element is absent");
                result_value += (*self_value) * (*other_value);
            }
            let result_position = Position(i / columns_number, i % columns_number);
            result.get_mut_elements().insert(result_position, result_value);
        }
        Ok(result.into_matrix())
    }


    fn transpose(&self) -> Self
        where Self::Value: Copy,
              Self: Clone,
    {
        let mut result = self.clone();
        result.get_mut_shape().swap_rows_number_and_columns_number();
        result.get_mut_elements().clear();
        for (position, value) in self.get_elements().iter()
        {
            let mut pos = position.clone();
            pos.swap_row_and_column();
            result.get_mut_elements().insert(pos, *value);
        }
        result
    }


    fn remove_row(&self, row: usize) -> Result<Matrix<Self::Value>, String>
        where Self::Value: Copy,
    {
        if row > self.get_shape().0 - 1
        {
            return Err(format!("Number of rows less than {}!", row + 1));
        }
        let shape = NewShape(self.get_shape().0 - 1, self.get_shape().1);
        let mut elements = HashMap::new();
        for (position, value) in self.get_elements()
        {
            if position.0 < row
            {
                elements.insert(position.clone(), *value);
            }
            if position.0 > row
            {
                let pos = Position(position.0 - 1, position.1);
                elements.insert(pos, *value);
            } 
        }
        let result = Matrix { shape, elements };
        Ok(result)
    }


    fn remove_column(&self, column: usize) -> Result<Matrix<Self::Value>, String>
        where Self::Value: Copy,
    {
        if column > self.get_shape().1 - 1
        {
            return Err(format!("Number of columns less than {}!", column + 1));
        }
        let shape = NewShape(self.get_shape().0, self.get_shape().1 - 1);
        let mut elements = HashMap::new();
        for (position, value) in self.get_elements()
        {
            if position.1 < column
            {
                elements.insert(position.clone(), *value);
            }
            if position.1 > column
            {
                let pos = Position(position.0, position.1 - 1);
                elements.insert(pos, *value);
            } 
        }
        let result = Matrix { shape, elements };
        Ok(result)
    }


    fn show<F>(&self, f: F)
        where F: Fn(&str),
              Self::Value: Copy + Debug
    {
        let NewShape(rows_number, columns_numbers) = self.get_shape();
        for row in 0..*rows_number
        {
            let mut row_str = String::from("[");
            for column in 0..*columns_numbers
            {
                let pos = Position(row, column);
                let value = self.get_element_value(&pos)
                    .expect("Element is absent");
                row_str += &format!("{:?}, ", value);
            }
            row_str = row_str[..row_str.len() - 2].to_string();
            row_str += "]";
            f(&format!("{}", row_str));
        }
    }
}
