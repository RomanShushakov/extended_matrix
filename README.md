## Extended matrix

A matrix calculation module.

### Example

```rust
#[macro_use]
extern crate extended_matrix;

use extended_matrix::{Matrix, BasicOperationsTrait};

let m_1 = Matrix::create(2, 2, &[1.0, 2.0, 3.0, 4.0]);
let m_2 = Matrix::create(2, 2, &[1.0, 2.0, 3.0, 4.0]);

let m_expected_1 = Matrix::create(2, 2, &[2.0, 4.0, 6.0, 8.0]);

assert_eq!(m_1.add(&m_2), Ok(m_expected_1));
```
