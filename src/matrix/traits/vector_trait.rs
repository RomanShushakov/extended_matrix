use extended_matrix_float::MyFloatTrait;

use crate::BasicOperationsTrait;
use crate::FloatTrait;
use crate::enums::Operation;
use crate::matrix::{NewShape, Position};


pub trait VectorTrait: BasicOperationsTrait
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
        where V: BasicOperationsTrait<Value = <Self as BasicOperationsTrait>::Value>,
              <Self as BasicOperationsTrait>::Value: FloatTrait<Output = <Self as BasicOperationsTrait>::Value>
    {
        self.shape_conformity_check(other, Operation::Addition)?;
        let NewShape(rows_number, columns_number) = self.get_shape();
        let mut result = <<Self as BasicOperationsTrait>::Value>::from(0f32);
        for i in 0..*rows_number
        {
            for j in 0..*columns_number
            {
                result += *self.get_element_value(&Position(i, j))? * 
                    *other.get_element_value(&Position(i, j))?;
            }
        }
        Ok(result)
    }

}
