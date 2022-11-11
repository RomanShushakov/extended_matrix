use crate::matrix::{Matrix, BasicOperationsTrait};


pub trait IntoMatrixTrait: BasicOperationsTrait
{
    fn into_matrix(&self) -> Matrix<<Self as BasicOperationsTrait>::Value>
        where <Self as BasicOperationsTrait>::Value: Copy
    {
        Matrix { shape: self.get_shape().clone(), elements: self.get_elements().clone() }
    }
}
