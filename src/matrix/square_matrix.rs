//! `SquareMatrix` type + implementations.
//!
//! `SquareMatrix` is the main “algorithm host” for dense square matrices in this crate.
//! Many operations delegate to trait methods from `matrix::traits` to keep the API surface tidy.


// external imports
use std::collections::HashMap;
use std::ops::AddAssign;

use crate::{
    BasicOperationsTrait, IntoMatrixTrait, SquareMatrixTrait, TryIntoSymmetricCompactedMatrixTrait,
};
use crate::{Position, Shape};

#[derive(PartialEq, Debug, Clone)]
pub struct SquareMatrix<V> {
    pub(crate) shape: Shape,
    pub(crate) elements: HashMap<Position, V>,
}

impl<V> BasicOperationsTrait for SquareMatrix<V> {
    type Value = V;

    fn get_shape(&self) -> &Shape {
        &self.shape
    }

    fn get_mut_shape(&mut self) -> &mut Shape {
        &mut self.shape
    }

    fn get_elements(&self) -> &HashMap<Position, Self::Value> {
        &self.elements
    }

    fn get_mut_elements(&mut self) -> &mut HashMap<Position, Self::Value> {
        &mut self.elements
    }
}

impl<V> IntoMatrixTrait for SquareMatrix<V> {}

impl<V> SquareMatrixTrait for SquareMatrix<V> {}

impl<V> TryIntoSymmetricCompactedMatrixTrait for SquareMatrix<V> {}

impl<V> SquareMatrix<V>
where
    V: Copy + From<f32> + PartialEq + AddAssign,
{
    pub fn create(order: usize, elements_values: &[V]) -> Self {
        let mut elements = HashMap::new();

        // sparse-by-default when element_values is empty
        if elements_values.is_empty() {
            return SquareMatrix {
                shape: Shape(order, order),
                elements,
            };
        }

        // dense fill (explicit values, missing -> 0)
        for i in 0..order * order {
            let (row_number, column_number) = (i / order, i % order);
            let position = Position(row_number, column_number);

            match elements_values.get(i) {
                Some(v) => elements.insert(position, *v),
                None => elements.insert(position, V::from(0f32)),
            };
        }

        SquareMatrix {
            shape: Shape(order, order),
            elements,
        }
    }

    pub fn add_value(&mut self, position: Position, delta: V) {
        if delta == V::from(0f32) {
            return;
        }

        let entry = self.elements.entry(position).or_insert(V::from(0f32));
        *entry += delta;

        if *entry == V::from(0f32) {
            self.elements.remove(&position);
        }
    }

    pub fn to_dense_values(&self) -> Vec<V> {
        let n = self.shape.0;
        let mut dense = vec![V::from(0f32); n * n];

        for (pos, val) in self.elements.iter() {
            dense[pos.0 * n + pos.1] = *val;
        }

        dense
    }

    pub fn to_dense(&self) -> SquareMatrix<V> {
        let n = self.shape.0;
        let dense = self.to_dense_values();
        SquareMatrix::create(n, &dense)
    }
}
