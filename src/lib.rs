mod matrix;
pub use matrix::
{
    Matrix, Position, BasicOperationsTrait, SquareMatrix, Vector3, VectorTrait, Vector, SquareMatrixTrait,
    TryIntoSquareMatrixTrait, TryIntoSymmetricCompactedMatrixTrait, Shape,
};
pub(crate) use matrix::IntoMatrixTrait;
pub(crate) use matrix::Operation;

mod traits;
pub use traits::FloatTrait;

mod tests;
