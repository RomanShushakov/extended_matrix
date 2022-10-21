use crate::enums::Operation;
use crate::extended_matrix::ExtendedMatrix;
use crate::shape::Shape;


const TOLERANCE: f32 = 1e-12;


#[test]
fn test_matrices_dimensions_conformity_check() -> Result<(), String>
{
    let m_1 = ExtendedMatrix::create(3, 2, 
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], TOLERANCE)?;
    let m_2 = ExtendedMatrix::create(2, 3, 
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], TOLERANCE)?;
    
    let addition = Operation::Addition;
    let multiplication = Operation::Multiplication;

    assert_eq!(m_1.matrices_dimensions_conformity_check(&m_2, addition), 
        Err("Extended matrix: Shapes of matrices do not conform to each other!".to_string()));

    let matrix_shape = Shape(3, 3);
    let matrix_basic_dimension = 2;
    assert_eq!(m_1.matrices_dimensions_conformity_check(&m_2, multiplication),
        Ok((matrix_basic_dimension, matrix_shape)));

    Ok(())
}
