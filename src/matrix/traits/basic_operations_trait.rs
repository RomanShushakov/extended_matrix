use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};

use crate::matrix::{NewShape, Position};
use crate::enums::Operation;


pub(crate) trait BasicOperationsTrait 
{
    type Other;
    type Value;

    fn get_shape(&self) -> &NewShape;
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
}
