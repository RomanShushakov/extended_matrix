pub mod basic_matrix;
pub mod extended_matrix;
pub mod matrix_element_position;
pub mod functions;

mod matrix;
pub use matrix::
{
    Matrix, Position, BasicOperationsTrait, SquareMatrix, Vector3, VectorTrait, Vector, SquareMatrixTrait,
    TryIntoSquareMatrixTrait, SymmetricMatrix
};
pub(crate) use matrix::IntoMatrixTrait;

mod enums;

mod traits;
pub use traits::{UIntTrait, FloatTrait};

mod tests;
