use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign, Mul, MulAssign};

use crate::matrix::{NewShape, Position};
use crate::enums::Operation;


pub trait BasicOperationsTrait 
{
    type Other;
    type Value;

    fn get_shape(&self) -> &NewShape;
    fn get_mut_shape(&mut self) -> &mut NewShape;
    fn get_elements(&self) -> &HashMap<Position, Self::Value>;
    fn get_mut_elements(&mut self) -> &mut HashMap<Position, Self::Value>;


    fn get_element_value(&self, position: &Position) -> &Self::Value
        where Self::Value: Copy
    {
        self.get_elements().get(position).expect("Element is absent")
    }


    fn get_mut_element_value(&mut self, position: &Position) -> &mut Self::Value
        where Self::Value: Copy
    {
        self.get_mut_elements().get_mut(position).expect("Element is absent")
    }


    fn shape_conformity_check(&self, other: &Self::Other, operation: Operation) -> Result<(), String>
        where Self::Other: BasicOperationsTrait
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


    fn add(&self, other: &Self::Other) -> Result<Self, String>
        where Self::Other: BasicOperationsTrait<Value = Self::Value>, 
              Self::Value: Copy + AddAssign,
              Self: Clone,
    {
        self.shape_conformity_check(&other, Operation::Addition)?;
        let mut result = self.clone();
        for (position, value) in other.get_elements()
        {
            *result.get_mut_element_value(position) += *value;
        }
        Ok(result)
    }


    fn subtract(&self, other: &Self::Other) -> Result<Self, String>
        where Self::Other: BasicOperationsTrait<Value = Self::Value>, 
              Self::Value: Copy + SubAssign,
              Self: Clone,
    {
        self.shape_conformity_check(&other, Operation::Subtraction)?;
        let mut result = self.clone();
        for (position, value) in other.get_elements()
        {
            *result.get_mut_element_value(position) -= *value;
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


    fn multiply(&self, other: &Self::Other) -> Result<Self, String>
        where Self::Other: BasicOperationsTrait<Value = Self::Value>, 
              Self::Value: Copy + AddAssign + SubAssign + Mul<Output = Self::Value> + From<f32>,
              Self: Clone,
    {
        self.shape_conformity_check(&other, Operation::Multiplication)?;
        let mut result = self.clone();
        let (rows_number, columns_number) = (self.get_shape().0, other.get_shape().1);
        result.get_mut_shape().update(rows_number, columns_number);
        result.get_mut_elements().clear();

        for i in 0..(rows_number * columns_number)
        {
            let mut result_value = <Self::Value>::from(0f32);
            for k in 0..self.get_shape().1
            {
                let self_position = Position(i / columns_number, k);
                let self_value = self.get_element_value(&self_position);
                let other_position = Position(k, i % columns_number);
                let other_value = other.get_element_value(&other_position);
                result_value += (*self_value) * (*other_value);
            }
            let result_position = Position(i / columns_number, i % columns_number);
            result.get_mut_elements().insert(result_position, result_value);
        }
        Ok(result)
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
}
