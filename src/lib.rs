mod matrix;
pub(crate) use matrix::IntoMatrixTrait;
pub use matrix::{
    BasicOperationsTrait, CsrMatrix, Matrix, Operation, Position, Shape, SquareMatrix,
    SquareMatrixTrait, TryIntoSquareMatrixTrait, TryIntoSymmetricCompactedMatrixTrait, Vector,
    Vector3, VectorTrait,
};

mod traits;
pub use traits::FloatTrait;

mod tests;
