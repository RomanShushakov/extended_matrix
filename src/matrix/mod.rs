mod matrix;
mod structs;
mod traits;
mod square_matrix;
mod vector_3;
mod vector;
mod symmetric_matrix;

pub use matrix::Matrix;
pub(crate) use structs::{Shape, NewShape};
pub use structs::Position;
pub use traits::{BasicOperationsTrait, VectorTrait, SquareMatrixTrait};
pub use square_matrix::SquareMatrix;
pub(crate) use traits::IntoMatrixTrait;
pub use vector_3::Vector3;
pub use vector::Vector;
pub use traits::TryIntoSquareMatrixTrait;
pub use symmetric_matrix::SymmetricMatrix;
