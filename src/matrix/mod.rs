mod matrix;
mod structs;
mod traits;
mod square_matrix;

pub use matrix::Matrix;
pub(crate) use structs::{Shape, NewShape};
pub use structs::Position;
pub use traits::BasicOperationsTrait;
pub use square_matrix::SquareMatrix;
pub(crate) use traits::IntoMatrixTrait;
