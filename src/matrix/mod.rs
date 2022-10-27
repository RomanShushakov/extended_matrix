mod matrix;
mod structs;
mod traits;
mod square_matrix;
mod vector_3;

pub use matrix::Matrix;
pub(crate) use structs::{Shape, NewShape};
pub use structs::Position;
pub use traits::{BasicOperationsTrait, VectorTrait};
pub use square_matrix::SquareMatrix;
pub(crate) use traits::IntoMatrixTrait;
pub use vector_3::Vector3;
