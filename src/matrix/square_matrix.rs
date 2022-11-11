use std::collections::HashMap;

use crate::matrix::{Position, NewShape};
use crate::matrix::{BasicOperationsTrait, IntoMatrixTrait, SquareMatrixTrait, TryIntoSymmetricMatrixTrait};


#[derive(PartialEq, Debug, Clone)]
pub struct SquareMatrix<V>
{
    pub(crate) shape: NewShape,
    pub(crate) elements: HashMap<Position, V>,
}


impl<V> BasicOperationsTrait for SquareMatrix<V>
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


impl<V> IntoMatrixTrait for SquareMatrix<V> {}


impl<V> SquareMatrixTrait for SquareMatrix<V> {}


impl<V> TryIntoSymmetricMatrixTrait for SquareMatrix<V> {}


impl<V> SquareMatrix<V> 
    where V: Copy + From<f32>,
{
    pub fn create(order: usize, elements_values: &[V]) -> Self
    {
        let mut elements = HashMap::new();

        for i in 0..order * order
        {
            let (row_number, column_number) = (i / order, i % order);
            let position = Position(row_number, column_number);

            match elements_values.get(i)
            {
                Some(v) => elements.insert(position, *v),
                None => elements.insert(position, V::from(0f32)),
            };
        }

        SquareMatrix { shape: NewShape(order, order), elements }
    }
}
