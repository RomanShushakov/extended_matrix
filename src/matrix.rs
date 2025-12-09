mod enums;
mod matrix;
mod square_matrix;
mod structs;
mod traits;
mod vector;
mod vector_3;

pub(crate) use enums::Operation;
pub use matrix::Matrix;
pub use square_matrix::SquareMatrix;
pub use structs::{Position, Shape};
pub(crate) use traits::IntoMatrixTrait;
pub use traits::{
    BasicOperationsTrait, SquareMatrixTrait, TryIntoSquareMatrixTrait,
    TryIntoSymmetricCompactedMatrixTrait, VectorTrait,
};
pub use vector::Vector;
pub use vector_3::Vector3;
