use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Sub, Mul};

use crate::FloatTrait;
use crate::{Matrix, Shape, Position};
use crate::{BasicOperationsTrait, IntoMatrixTrait, VectorTrait};


#[derive(Debug, PartialEq, Clone)]
pub struct Vector3<V> 
{
    pub(crate) shape: Shape,
    pub(crate) elements: HashMap<Position, V>,
}


impl<V> BasicOperationsTrait for Vector3<V>
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


impl<V> IntoMatrixTrait for Vector3<V> {}


impl<V> VectorTrait for Vector3<V> {}


impl<V> Vector3<V> 
    where V: Debug + Copy + From<f32>,
{
    pub fn create(components: &[V; 3]) -> Self
    {
        let shape = Shape(3, 1);
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


    pub fn cosine_angle_between_vectors(&self, other: &Self) -> V
        where V: FloatTrait<Output = V>
    {
        self.dot_product(other).expect("Dot product could not be calculated") / 
            (self.norm().expect("Norm could not be calculated") * 
            other.norm().expect("Norm could not be calculated"))
    }


    pub fn projection_perpendicular_to_vector(&self, other: &Self) -> Self
        where V: FloatTrait<Output = V>
    {
        self.cross_product(other)
            .multiply_by_scalar(V::from(-1f32) / other.norm().expect("Norm could not be calculated"))
            .cross_product(other)
            .multiply_by_scalar(V::from(1f32) / other.norm().expect("Norm could not be calculated"))
    }


    fn check_vectors_lenghts_are_the_same(&self, other: &Self, rel_tol: V) -> Result<(), String>
        where V: FloatTrait<Output = V>
    {
        let (min_length, max_length) = 
            {
                let (lhs_norm, rhs_norm) = (self.norm()?, other.norm()?);
                if lhs_norm < rhs_norm { (lhs_norm, rhs_norm) } else { (rhs_norm, lhs_norm) }
            };
        if (max_length - min_length) / min_length > rel_tol
        {
            return Err("Vectors with different lenghts could not be aligned".to_string());
        }
        Ok(())
    }


    pub fn rotation_matrix_to_align_with_vector(&self, other: &Self, rel_tol: V, abs_tol: V) -> Result<Matrix<V>, String>
        where V: FloatTrait<Output = V>
    {
        self.check_vectors_lenghts_are_the_same(other, rel_tol)?;
        let c = self.cosine_angle_between_vectors(other);
        if V::from(1f32) - c < abs_tol
        {
            return Ok(Matrix::create(3, 3, &[
                V::from(1.0), V::from(0.0), V::from(0.0),
                V::from(0.0), V::from(1.0), V::from(0.0),
                V::from(0.0), V::from(0.0), V::from(1.0),
            ]));
        }
        if V::from(1f32) + c < abs_tol
        {
            if self.get_components()[..2] == [V::from(0f32); 2] && other.get_components()[..2] == [V::from(0f32); 2]
            {
                return Ok(Matrix::create(3, 3, &[
                    V::from(-1.0), V::from(0.0), V::from(0.0),
                    V::from(0.0), V::from(1.0), V::from(0.0),
                    V::from(0.0), V::from(0.0), V::from(-1.0),
                ]));
            }
            return Ok(Matrix::create(3, 3, &[
                V::from(-1.0), V::from(0.0), V::from(0.0),
                V::from(0.0), V::from(-1.0), V::from(0.0),
                V::from(0.0), V::from(0.0), V::from(-1.0),
            ]));
        }
        let axis = self.cross_product(other);
        let axis_norm = axis.norm()?;
        let [x, y, z] = axis.get_components();
        let [x_n, y_n, z_n] = [x / axis_norm, y / axis_norm, z / axis_norm];
        let s = axis.norm()? / (self.norm()? * other.norm()?);
        let t = V::from(1f32) - c;
        let rotation_matrix = Matrix::create(3, 3, 
            &[
                t * x_n * x_n + c, t * x_n * y_n - z_n * s, t * x_n * z_n + y_n * s,
                t * x_n * y_n + z_n * s	, t * y_n * y_n + c, t * y_n * z_n - x_n * s,
                t * x_n * z_n - y_n * s, t * y_n * z_n + x_n * s, t * z_n * z_n + c, 
            ]);
        Ok(rotation_matrix)
    }
}
