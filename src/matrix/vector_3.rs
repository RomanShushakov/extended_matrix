use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Sub, Mul};

use crate::matrix::{Matrix, NewShape, Position};
use crate::matrix::{BasicOperationsTrait, IntoMatrixTrait, VectorTrait};
use crate::enums::Operation;


#[derive(Debug, PartialEq, Clone)]
pub struct Vector3<V> 
{
    pub(crate) shape: NewShape,
    pub(crate) elements: HashMap<Position, V>,
}


impl<V> BasicOperationsTrait for Vector3<V>
{
    type Value = V;

    fn get_shape(&self) -> &NewShape 
    {
        &self.shape
    }


    fn get_mut_shape(&mut self) -> &mut NewShape 
    {
        &mut self.shape
    }


    fn get_elements(&self) -> &HashMap<Position, Self::Value> 
    {
        &self.elements
    }


    fn get_mut_elements(&mut self) -> &mut HashMap<Position, Self::Value> 
    {
        &mut self.elements
    }
}


impl<V> IntoMatrixTrait for Vector3<V>
{
    type Value = V;

    fn into_matrix(self) -> Matrix<Self::Value>
    {
        Matrix { shape: self.shape, elements: self.elements }
    }
}


impl<V> VectorTrait for Vector3<V> {}


impl<V> Vector3<V> 
    where V: Debug + Copy + From<f32>,
{
    pub fn create(components: &[V; 3]) -> Self
    {
        let shape = NewShape(3, 1);
        let elements = HashMap::from([
            (Position(0, 0), components[0]), (Position(1, 0), components[1]), (Position(2, 0), components[2])]);

        Vector3 { shape, elements }
    }


    pub fn cross_product<M>(&self, other: &M) -> Result<Self, String>
        where V: Sub<Output = V> + Mul<Output = V>,
              M: BasicOperationsTrait<Value = V>,
    {
        self.shape_conformity_check(other, Operation::Addition)?;
        let mut result = self.clone();
        if self.shape.1 == 1
        {
            *result.get_mut_element_value(&Position(0, 0))? = 
                *self.get_element_value(&Position(1, 0))? * *other.get_element_value(&Position(2, 0))? -
                *self.get_element_value(&Position(2, 0))? * *other.get_element_value(&Position(1, 0))?;
            *result.get_mut_element_value(&Position(1, 0))? = 
                *self.get_element_value(&Position(2, 0))? * *other.get_element_value(&Position(0, 0))? -
                *self.get_element_value(&Position(0, 0))? * *other.get_element_value(&Position(2, 0))?;
            *result.get_mut_element_value(&Position(2, 0))? = 
                *self.get_element_value(&Position(0, 0))? * *other.get_element_value(&Position(1, 0))? -
                *self.get_element_value(&Position(1, 0))? * *other.get_element_value(&Position(0, 0))?;
        }
        else
        {
            *result.get_mut_element_value(&Position(0, 0))? = 
                *self.get_element_value(&Position(0, 1))? * *other.get_element_value(&Position(0, 2))? -
                *self.get_element_value(&Position(0, 2))? * *other.get_element_value(&Position(0, 1))?;
            *result.get_mut_element_value(&Position(0, 1))? = 
                *self.get_element_value(&Position(0, 2))? * *other.get_element_value(&Position(0, 0))? -
                *self.get_element_value(&Position(0, 0))? * *other.get_element_value(&Position(0, 2))?;
            *result.get_mut_element_value(&Position(0, 2))? = 
                *self.get_element_value(&Position(0, 0))? * *other.get_element_value(&Position(0, 1))? -
                *self.get_element_value(&Position(0, 1))? * *other.get_element_value(&Position(0, 0))?;
        }
        
        Ok(result)
    }
}
