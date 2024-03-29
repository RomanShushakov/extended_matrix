use std::collections::HashMap;
use std::fmt::Debug;

use crate::{Shape, Position};
use crate::{BasicOperationsTrait, IntoMatrixTrait, TryIntoSquareMatrixTrait};


#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<V> 
{
    pub(crate) shape: Shape,
    pub(crate) elements: HashMap<Position, V>,
}


impl<V> BasicOperationsTrait for Matrix<V>
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


impl<V> IntoMatrixTrait for Matrix<V> {}


impl<V> TryIntoSquareMatrixTrait for Matrix<V> {}


impl<V> Matrix<V> 
    where V: Debug + Copy + From<f32>,
{
    pub fn create(rows_number: usize, columns_number: usize, elements_values: &[V]) -> Self
    {
        let shape = Shape(rows_number, columns_number);
        let mut elements = HashMap::new();

        for i in 0..rows_number * columns_number
        {
            let (row_number, column_number) = (i / columns_number, i % columns_number);
            let position = Position(row_number, column_number);

            match elements_values.get(i)
            {
                Some(v) => elements.insert(position, *v),
                None => elements.insert(position, V::from(0f32)),
            };
        }

        Matrix { shape, elements }
    }
}
