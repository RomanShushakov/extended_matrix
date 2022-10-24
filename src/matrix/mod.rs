mod matrix;
mod structs;
mod traits;

pub use matrix::Matrix;
pub(crate) use structs::{Shape, NewShape};
pub use structs::Position;
pub use traits::BasicOperationsTrait;