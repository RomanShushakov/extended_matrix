use std::collections::HashMap;
use std::fmt::Debug;

use crate::matrix::{Matrix, NewShape, Position};
use crate::matrix::{BasicOperationsTrait, IntoMatrixTrait, VectorTrait};


#[derive(Debug, PartialEq, Clone)]
pub struct Vector3<V> 
{
    pub(crate) shape: NewShape,
    pub(crate) elements: HashMap<Position, V>,
}


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
