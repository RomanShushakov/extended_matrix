//! # extended_matrix
//!
//! A small linear-algebra playground focused on *how* basic matrix operations work under the hood.
//! The crate provides dense matrices/vectors plus a “square matrix” toolbox (Gaussian elimination,
//! LU/LUP decomposition, determinants, inverses, etc.).
//!
//! This project started as a learning-by-building exercise: implement core routines explicitly,
//! keep the code readable, and expose the algorithms through a compact API.
//!
//! If you need a production-grade BLAS/LAPACK-backed stack, reach for `nalgebra`/`ndarray` + friends.
//! If you want approachable implementations you can step through and experiment with, this crate is
//! meant to be useful.


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
