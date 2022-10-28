use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Sub, Mul, MulAssign, Div};

use crate::FloatTrait;
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


    pub fn get_components(&self) -> [V; 3]
    {
        let mut components = [V::from(0f32); 3];
        for row in 0..self.shape.0
        {
            for column in 0..self.shape.1
            {
                components[row + column] = *self.get_element_value(&Position(row, column))
                    .expect("Element is absent");
            }
        }
        components
    }


    pub fn cross_product(&self, other: &Self) -> Self
        where V: Sub<Output = V> + Mul<Output = V>
    {
        let mut lhs = self.clone();
        if lhs.shape.0 == 1
        {
            lhs = lhs.transpose();
        } 
        let mut rhs = other.clone();
        if rhs.shape.0 == 1
        {
            rhs = rhs.transpose();
        }
        let mut result = lhs.clone();
        *result.get_mut_element_value(&Position(0, 0)).expect("Element is absent") = 
            *lhs.get_element_value(&Position(1, 0)).expect("Element is absent") * 
            *rhs.get_element_value(&Position(2, 0)).expect("Element is absent") -
            *lhs.get_element_value(&Position(2, 0)).expect("Element is absent") * 
            *rhs.get_element_value(&Position(1, 0)).expect("Element is absent");
        *result.get_mut_element_value(&Position(1, 0)).expect("Element is absent") = 
            *lhs.get_element_value(&Position(2, 0)).expect("Element is absent") * 
            *rhs.get_element_value(&Position(0, 0)).expect("Element is absent") -
            *lhs.get_element_value(&Position(0, 0)).expect("Element is absent") * 
            *rhs.get_element_value(&Position(2, 0)).expect("Element is absent");
        *result.get_mut_element_value(&Position(2, 0)).expect("Element is absent") = 
            *lhs.get_element_value(&Position(0, 0)).expect("Element is absent") * 
            *rhs.get_element_value(&Position(1, 0)).expect("Element is absent") -
            *lhs.get_element_value(&Position(1, 0)).expect("Element is absent") * 
            *rhs.get_element_value(&Position(0, 0)).expect("Element is absent");
        result
    }


    pub fn angle_between_vectors(&self, other: &Self) -> V
        where V: FloatTrait<Output = V>
    {
        let cos_angle = self.dot_product(other).expect("Dot product could not be calculated") / 
            (self.norm().expect("Norm could not be calculated") * 
            other.norm().expect("Norm could not be calculated"));
        cos_angle.my_acos()
    }


    pub fn projection_perpendicular_to_vector(&self, other: &Self) -> Self
        where V: FloatTrait<Output = V>
    {
        self.cross_product(other)
            .multiply_by_scalar(V::from(-1f32) / other.norm().expect("Norm could not be calculated"))
            .cross_product(other)
            .multiply_by_scalar(V::from(1f32) / other.norm().expect("Norm could not be calculated"))
    }
}
