mod matrix;
pub(crate) use matrix::IntoMatrixTrait;
pub(crate) use matrix::Operation;
pub use matrix::{
    BasicOperationsTrait, Matrix, Position, Shape, SquareMatrix, SquareMatrixTrait,
    TryIntoSquareMatrixTrait, TryIntoSymmetricCompactedMatrixTrait, Vector, Vector3, VectorTrait,
};

mod traits;
pub use traits::FloatTrait;

mod tests;
