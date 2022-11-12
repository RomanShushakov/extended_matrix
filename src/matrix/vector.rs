use std::collections::HashMap;
use std::fmt::Debug;

use crate::{Shape, Position};
use crate::{BasicOperationsTrait, IntoMatrixTrait, VectorTrait};


#[derive(Debug, PartialEq, Clone)]
pub struct Vector<V> 
{
    pub(crate) shape: Shape,
    pub(crate) elements: HashMap<Position, V>,
}


impl<V> BasicOperationsTrait for Vector<V>
{
    type Value = V;

    fn get_shape(&self) -> &Shape 
    {
        &self.shape
    }


    fn get_mut_shape(&mut self) -> &mut Shape 
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


impl<V> IntoMatrixTrait for Vector<V> {}


impl<V> VectorTrait for Vector<V> {}


impl<V> Vector<V> 
    where V: Debug + Copy + From<f32>,
{
    pub fn create(values: &[V]) -> Self
    {

        let shape = Shape(values.len(), 1);
        let elements = values.iter().enumerate().map(|(i, v)| (Position(i, 0), *v))
            .collect::<HashMap<Position, V>>();
        Vector { shape, elements }
    }
}
