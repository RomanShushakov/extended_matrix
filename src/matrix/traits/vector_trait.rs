use extended_matrix_float::MyFloatTrait;

use crate::{BasicOperationsTrait, IntoMatrixTrait};
use crate::FloatTrait;
use crate::matrix::Position;


pub trait VectorTrait: 
    BasicOperationsTrait + 
    IntoMatrixTrait<Value = <Self as BasicOperationsTrait>::Value>
{
    fn vector_shape_conformity_check(&self) -> Result<(), String>
    {
        if self.get_shape().0 != 1 && self.get_shape().1 != 1
        {
            return Err("Not a vector".to_string())
        }
        Ok(())
    }


    fn norm(&self) -> Result<<Self as BasicOperationsTrait>::Value, String>
        where <Self as BasicOperationsTrait>::Value: FloatTrait<Output = <Self as BasicOperationsTrait>::Value>
    {
        self.vector_shape_conformity_check()?;
        Ok(self.get_elements()
            .values()
            .fold(<<Self as BasicOperationsTrait>::Value>::from(0f32),
                |acc, x| 
                    acc + *x * *x)
            .my_sqrt())
    }


    fn dot_product(&self, other: &Self) -> Result<<Self as BasicOperationsTrait>::Value, String>
        where <Self as BasicOperationsTrait>::Value: FloatTrait<Output = <Self as BasicOperationsTrait>::Value>,
              Self: Clone
    {
        self.vector_shape_conformity_check()?;
        other.vector_shape_conformity_check()?;
        if self.get_shape().1 == 1 && other.get_shape().1 == 1
        {
            Ok(*self.transpose().multiply(other)?.get_element_value(&Position(0, 0))?)
        }
        else if self.get_shape().0 == 1 && other.get_shape().1 == 1
        {
            Ok(*self.multiply(other)?.get_element_value(&Position(0, 0))?)
        }
        else if self.get_shape().0 == 1 && other.get_shape().0 == 1
        {
            Ok(*self.multiply(&other.transpose())?.get_element_value(&Position(0, 0))?)
        }
        else
        {
            Ok(*other.multiply(self)?.get_element_value(&Position(0, 0))?)
        }
    }
}
