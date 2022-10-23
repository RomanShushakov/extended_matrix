use std::collections::HashMap;

use crate::Matrix;
use crate::matrix::{NewShape, Position};


#[test]
fn test_matrix_create()
{
    let m = Matrix::create(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

    let expected = Matrix 
        { 
            shape: NewShape(2, 2), 
            elements: HashMap::from([(Position(0, 0), 1.0), (Position(0, 1), 2.0), (Position(1, 0), 3.0), 
                (Position(1, 1), 4.0)]) 
        };

    assert_eq!(m, expected);
}
