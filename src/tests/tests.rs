use crate::enums::Operation;
use crate::extended_matrix::ExtendedMatrix;
use crate::shape::Shape;


const TOLERANCE: f32 = 1e-12;


#[test]
fn test_matrices_dimensions_conformity_check() -> Result<(), String>
{
    let m_1 = ExtendedMatrix::create(3u8, 2, 
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


#[test]
fn test_add_matrix() -> Result<(), String>
{
    let m_1 = ExtendedMatrix::create(2u16, 2, 
        vec![1.0, 2.0, 3.0, 4.0], TOLERANCE)?;
    let m_2 = ExtendedMatrix::create(2, 2, 
        vec![5.0, 6.0, 7.0, 8.0], TOLERANCE)?;

    let mut result = m_1.add_matrix(&m_2)?;
    result.try_to_symmetrize(TOLERANCE);

    let expected = ExtendedMatrix::create(2, 2, 
        vec![6.0, 8.0, 10.0, 12.0], TOLERANCE)?;

    assert_eq!(result, expected);

    Ok(())
}


#[test]
fn test_subtract_matrix() -> Result<(), String>
{
    let m_1 = ExtendedMatrix::create(2usize, 2, 
        vec![1.0, 2.0, 3.0, 4.0], TOLERANCE)?;
    let m_2 = ExtendedMatrix::create(2, 2, 
        vec![5.0, 6.0, 7.0, 8.0], TOLERANCE)?;

    let mut result = m_1.subtract_matrix(&m_2)?;
    result.try_to_symmetrize(TOLERANCE);

    let expected = ExtendedMatrix::create(2, 2, 
        vec![-4.0, -4.0, -4.0, -4.0], TOLERANCE)?;

    assert_eq!(result, expected);

    Ok(())
}
