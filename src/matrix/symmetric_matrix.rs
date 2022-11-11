use std::collections::HashMap;

use crate::matrix::{Position, NewShape};
use crate::matrix::{BasicOperationsTrait, IntoMatrixTrait, SquareMatrixTrait};
use crate::FloatTrait;


#[derive(PartialEq, Debug, Clone)]
pub struct SymmetricMatrix<V>
{
    pub(crate) shape: NewShape,
    pub(crate) elements: HashMap<Position, V>,
}


impl<V> BasicOperationsTrait for SymmetricMatrix<V>
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


impl<V> IntoMatrixTrait for SymmetricMatrix<V> {}


impl<V> SquareMatrixTrait for SymmetricMatrix<V> {}


impl<V> SymmetricMatrix<V> 
    where V: FloatTrait<Output = V>,
{
    pub fn try_to_create(order: usize, elements_values: &[V], rel_tol: V) -> Result<Self, String>
    {
        let mut elements = HashMap::new();

        for i in 0..order * order
        {
            let (row_number, column_number) = (i / order, i % order);
            let position = Position(row_number, column_number);

            let mut value = match elements_values.get(i)
            {
                Some(v) => *v,
                None => V::from(0f32),
            };

            if row_number > column_number
            {
                let mut symm_position = position.clone();
                symm_position.swap_row_and_column();
                let symm_value = elements.get(&symm_position).expect("Element is absent");
                if (V::from(1f32) - value / *symm_value).my_abs() > rel_tol
                {
                    return Err(format!("Element [{row_number}, {column_number}] does not match with \
                        [{column_number}, {row_number}]!"));
                }
                value = *symm_value;

            }
            elements.insert(position, value);
        }

        Ok(SymmetricMatrix { shape: NewShape(order, order), elements })
    }


    pub fn force_create(order: usize, elements_values: &[V], rel_tol: V, warnings: &mut Vec<String>) -> Self
    {
        let mut elements = HashMap::new();

        for i in 0..order * order
        {
            let (row_number, column_number) = (i / order, i % order);
            let position = Position(row_number, column_number);

            let mut value = match elements_values.get(i)
            {
                Some(v) => *v,
                None => V::from(0f32),
            };

            if row_number > column_number
            {
                let mut symm_position = position.clone();
                symm_position.swap_row_and_column();
                let symm_value = elements.get(&symm_position).expect("Element is absent");
                if (V::from(1f32) - value / *symm_value).my_abs() > rel_tol
                {
                    let warning = format!("Element [{row_number}, {column_number}] does not match with \
                        [{column_number}, {row_number}]!");
                    warnings.push(warning);
                }
                value = *symm_value;
            }
            elements.insert(position, value);
        }

        SymmetricMatrix { shape: NewShape(order, order), elements }
    }
}
