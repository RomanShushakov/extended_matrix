# extended_matrix

![Rust](https://img.shields.io/badge/Rust-stable-orange)
![Linear Algebra](https://img.shields.io/badge/Linear%20Algebra-matrix%20basics-blue)
![Status](https://img.shields.io/badge/status-learning%20%2F%20building-lightgrey)

A small Rust crate I wrote while revisiting numerical linear algebra fundamentals.

The goal here is not to compete with established libraries, but to keep **core matrix routines readable and easy to step through** (Gaussian elimination, LU/LUP decomposition, determinant/inverse helpers, etc.). I’ve been using it as a supporting crate in my personal finite‑element / solver experiments, where it’s handy to have “plain” implementations I can inspect and tweak.

If you need production-grade performance, stability guarantees, and a broad ecosystem, you’ll likely want `nalgebra`, `ndarray`, or BLAS/LAPACK-backed solutions instead. This crate is more of a **learning + building block**.

## What’s inside

- **Dense matrices and vectors**
  - `Matrix`, `SquareMatrix`
  - basic arithmetic helpers and utilities
- **Square-matrix algorithms**
  - Gaussian elimination helpers (`eliminate_gep`, `substitute_gep`, `gauss_gep`)
  - LU / LUP decomposition (`lup_decomp`, `decompose_lup`)
  - determinant and inverse routines built on top of decompositions
- **Sparse matrix container**
  - `CsrMatrix` (lightweight CSR storage) — useful for experiments, not a full sparse toolkit

## Quick example

```rust
use extended_matrix::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 2x2 system: A x = b
    let a = SquareMatrix::from_vec(vec![
        vec![2.0, 1.0],
        vec![5.0, 7.0],
    ])?;

    let b = Vector::from_vec(vec![11.0, 13.0])?;

    // Solve using Gaussian elimination (see the trait docs for variants)
    let x = a.gauss_gep(b)?;
    println!("x = {:?}", x);

    Ok(())
}
```

## Notes on the “*_gep” naming

Inside the square-matrix trait you’ll see methods suffixed with `*_gep`:
- `eliminate_gep`
- `substitute_gep`
- `gauss_gep`

In this codebase, `gep` is used as a short label for the “Gaussian elimination process” style helpers: elimination + back-substitution, typically with some form of pivoting/row handling depending on the method.

## Project philosophy

- Prefer straightforward code over clever tricks.
- Keep algorithms explicit (so it’s easy to reason about numerical behavior).
- Use small, composable helpers (decomposition routines power determinant/inverse/solves).

## License

MIT (see `LICENSE`).
