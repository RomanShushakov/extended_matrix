pub mod basic_matrix;
pub mod extended_matrix;
pub mod matrix_element_position;
pub mod functions;

mod matrix;
pub use matrix::Matrix;
pub use matrix::Position;

mod enums;

mod traits;
pub use traits::{UIntTrait, FloatTrait};

mod tests;
