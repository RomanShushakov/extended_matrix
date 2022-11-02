use std::collections::HashMap;
use std::fmt::Debug;

use crate::matrix::{Matrix, NewShape, Position};
use crate::matrix::{BasicOperationsTrait, IntoMatrixTrait, VectorTrait};


#[derive(Debug, PartialEq, Clone)]
pub struct Vector<V> 
{
    pub(crate) shape: NewShape,
    pub(crate) elements: HashMap<Position, V>,
}


impl<V> BasicOperationsTrait for Vector<V>
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


impl<V> IntoMatrixTrait for Vector<V>
{
    type Value = V;

    fn into_matrix(self) -> Matrix<Self::Value>
    {
        Matrix { shape: self.shape, elements: self.elements }
    }
}


impl<V> VectorTrait for Vector<V> {}


impl<V> Vector<V> 
    where V: Debug + Copy + From<f32>,
{
    pub fn create(values: &[V]) -> Self
    {

        let shape = NewShape(values.len(), 1);
        let elements = values.iter().enumerate().map(|(i, v)| (Position(i, 0), *v))
            .collect::<HashMap<Position, V>>();
        Vector { shape, elements }
    }
}
