// external imports
use extended_matrix_float::MyFloatTrait;

use crate::FloatTrait;
use crate::Position;
use crate::{BasicOperationsTrait, IntoMatrixTrait};

pub trait VectorTrait: IntoMatrixTrait {
    fn vector_shape_conformity_check(&self) -> Result<(), String> {
        if self.get_shape().0 != 1 && self.get_shape().1 != 1 {
            return Err("Not a vector".to_string());
        }
        Ok(())
    }

    fn norm(&self) -> Result<<Self as BasicOperationsTrait>::Value, String>
    where
        <Self as BasicOperationsTrait>::Value:
            FloatTrait<Output = <Self as BasicOperationsTrait>::Value>,
    {
        self.vector_shape_conformity_check()?;
        Ok(self
            .get_elements()
            .values()
            .fold(
                <<Self as BasicOperationsTrait>::Value>::from(0f32),
                |acc, x| acc + *x * *x,
            )
            .my_sqrt())
    }

    fn dot_product(&self, other: &Self) -> Result<<Self as BasicOperationsTrait>::Value, String>
    where
        <Self as BasicOperationsTrait>::Value:
            FloatTrait<Output = <Self as BasicOperationsTrait>::Value>,
        Self: Clone,
    {
        self.vector_shape_conformity_check()?;
        other.vector_shape_conformity_check()?;
        let mut lhs = self.clone();
        let mut rhs = other.clone();
        if lhs.get_shape().1 == 1 {
            lhs = lhs.transpose();
        }
        if rhs.get_shape().0 == 1 {
            rhs = rhs.transpose();
        }
        Ok(*lhs.multiply(&rhs)?.get_element_value(&Position(0, 0))?)
    }
}
