mod structs;
mod traits;
mod enums;
mod matrix;
mod square_matrix;
mod vector;
mod vector_3;

pub use matrix::Matrix;
pub use structs::{Position, Shape};
pub use traits::
{
    BasicOperationsTrait, VectorTrait, SquareMatrixTrait, TryIntoSquareMatrixTrait, 
    TryIntoSymmetricCompactedMatrixTrait,
};
pub(crate) use enums::Operation;
pub use square_matrix::SquareMatrix;
pub(crate) use traits::IntoMatrixTrait;
pub use vector::Vector;
pub use vector_3::Vector3;
