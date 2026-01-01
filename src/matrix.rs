//! Matrix module root.
//!
//! This module wires together the concrete matrix types and the algorithmic traits/implementations.
//! Most “interesting” routines live under `matrix::traits` and are implemented for `SquareMatrix`.
//!
//! High-level structure:
//! - `structs`: small helper structs (shape, position, …)
//! - `traits`: algorithm contracts (e.g. square-matrix solvers/decompositions)
//! - concrete types: `Matrix`, `SquareMatrix`, `CsrMatrix`
//! - implementations: trait impls + operator overloads


mod csr_matrix;
mod enums;
mod matrix;
mod square_matrix;
mod structs;
mod traits;
mod vector;
mod vector_3;

pub use csr_matrix::CsrMatrix;
pub use enums::Operation;
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
