use crate::matrix::Matrix;


pub trait IntoMatrixTrait
{
    type Value;

    fn into_matrix(self) -> Matrix<Self::Value>;
}
