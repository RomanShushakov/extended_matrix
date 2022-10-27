use extended_matrix_float::MyFloatTrait;

use crate::{BasicOperationsTrait, IntoMatrixTrait};
use crate::FloatTrait;
use crate::enums::Operation;
use crate::matrix::{NewShape, Position};


pub trait VectorTrait: BasicOperationsTrait + IntoMatrixTrait
{
    fn norm(&self) -> <Self as BasicOperationsTrait>::Value
        where <Self as BasicOperationsTrait>::Value: FloatTrait<Output = <Self as BasicOperationsTrait>::Value>
    {
        self.get_elements()
            .values()
            .fold(<<Self as BasicOperationsTrait>::Value>::from(0f32),
                |acc, x| 
                    acc + *x * *x)
            .my_sqrt()
    }


    fn dot_product<V>(&self, other: &V) -> Result<<Self as BasicOperationsTrait>::Value, String>
        where V: BasicOperationsTrait<Value = <Self as BasicOperationsTrait>::Value> + 
                 IntoMatrixTrait<Value = <Self as BasicOperationsTrait>::Value> + Clone,
              <Self as BasicOperationsTrait>::Value: FloatTrait<Output = <Self as BasicOperationsTrait>::Value>,
              Self: Clone
    {
        Ok(*self.transpose().multiply(other)?.get_element_value(&Position(0, 0))?)
    }
}
