
use extended_matrix::extended_matrix::{ExtendedMatrix};
use extended_matrix::functions::try_to_compact_matrix;
use extended_matrix::matrix_element_position::MatrixElementPosition;
use colsol::{factorization, find_unknown};

mod enums;
use crate::enums::Operation;
use crate::matrix::{BasicOperationsTrait, IntoMatrixTrait, SquareMatrixTrait};

mod matrix;
pub use matrix::{Matrix, SquareMatrix, Position, Vector3, VectorTrait, Vector};

mod traits;
pub use traits::{UIntTrait, FloatTrait};

use std::fmt::Debug;


fn main() -> Result<(), String>
{
    // let tolerance = 1e-15f64;
    // let f = |data: &str| println!("{}", data);

    // let mut m8 = ExtendedMatrix::create(3u32, 3u32,
    //     vec![
    //         3.0f64, -0.1, -0.2,
    //         -0.1, 7.0, -3.2,
    //         -0.2, -3.2, 10.0,
    //     ],
    //     tolerance).unwrap();

    // let (l8, u8) = m8.lu_decomposition().unwrap();

    // l8.show_matrix(f);
    // println!();
    // u8.show_matrix(f);
    // println!();

    // let b = ExtendedMatrix::create(3u32, 1u32,
    //     vec![
    //         7.85f64,
    //         -19.3,
    //         71.4,
    //     ],
    // tolerance).unwrap();

    // let r8 = m8.direct_solution(&b, true).unwrap();
    // r8.show_matrix(f);
    // println!();


    // let m1 = ExtendedMatrix::create(4u16, 4u16,
    //         vec![
    //             5.0f64, -4.0, 1.0, 0.0,
    //             -4.0, 6.0, -4.0, 1.0,
    //             1.0, -4.0, 6.0, -4.0,
    //             0.0, 1.0, -4.0, 5.0,
    //         ],
    //         tolerance
    //     )?;

    // let m2 = ExtendedMatrix::create(4u16, 1u16,
    //         vec![
    //             0.0,
    //             1.0,
    //             0.0,
    //             0.0,
    //         ],
    //         tolerance
    //     )?;

    // let result = m1.direct_solution(&m2, true)?;

    // result.show_matrix(f);
    // println!();


    // let m1 = ExtendedMatrix::create(5u16, 5u16,
    //     vec![
    //         2.0f64, -2.0, 0.0, 0.0, -1.0,
    //         -2.0, 3.0, -2.0, 0.0, 0.0,
    //         0.0, -2.0, 5.0, -3.0, 0.0,
    //         0.0, 0.0, -3.0, 10.0, 4.0,
    //         -1.0, 0.0, 0.0, 4.0, 10.0,
    //     ],
    //     tolerance
    // )?;

    // let m2 = ExtendedMatrix::create(5u16, 1u16,
    //         vec![
    //             0.0,
    //             1.0,
    //             0.0,
    //             0.0,
    //             0.0,
    //         ],
    //         tolerance
    //     )?;

    // let result = m1.direct_solution(&m2, true)?;

    // result.show_matrix(f);
    // println!();

    // // let lhs = ExtendedMatrix::create(12u32, 1u32,
    // // vec![
    // //     -0.001,
    // //     0.0,
    // //     0.0,
    // //     0.0,
    // //     0.0,
    // //     0.0,
    // //     0.001,
    // //     0.0,
    // //     0.0,
    // //     0.0,
    // //     0.0,
    // //     0.0,],
    // // tolerance)?;
    // //
    // // let rhs = ExtendedMatrix::create(1u32, 12u32,
    // //     vec![
    // //         -0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.001, 0.0, 0.0, 0.0, 0.0, 0.0, ],
    // //      tolerance)?;
    // //
    // // let result = lhs.multiply_by_matrix(&rhs)?;
    // // result.show_matrix(f);
    // // println!();

    // // let (mut a, maxa) = try_to_compact_matrix(&m1)?;
    // // println!("{:?}", maxa);
    // // println!("{:?}", a);
    // //
    // // let shape = m2.copy_shape();
    // // let mut v = Vec::new();
    // // let mut nn = 0i64;
    // // for row in 0..shape.0
    // // {
    // //     for column in 0..shape.1
    // //     {
    // //         let element_value = m2.copy_element_value_or_zero(
    // //             MatrixElementPosition::create(row, column))?;
    // //         v.push(element_value);
    // //         nn += 1;
    // //     }
    // // }
    // //
    // // println!("{:?}", v);
    // //
    // // factorization(&mut a, nn, &maxa)?;
    // // find_unknown(&a, &mut v, nn, &maxa);
    // //
    // // println!("{:?}", v);


    // let mut m50 = ExtendedMatrix::create(3u8, 3u8,
    //     vec![0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0], tolerance)?;
    // m50.show_matrix(f);
    // println!("{:?}", m50.ref_matrix_type());
    // println!();

    // m50.transpose();

    // m50.show_matrix(f);


    // let tolerance = 1e-15f64;
    // let f = |data: &str| println!("{}", data);

    // let mut m1 = ExtendedMatrix::create(2u32, 2u32,
    //     vec![
    //         0.0f64, 1.0, 
    //         -1.0, 0.0,
    //     ],
    //     tolerance).unwrap();

    // let m1_inv = m1.inverse()?;

    // m1_inv.show_matrix(f);
    // println!();

    // let m2 = m1.multiply_by_matrix(&m1_inv)?;
    // m2.show_matrix(f);

    // println!();

    // let det = m1.determinant_2x2()?;
    // println!("{}", det);
    // println!();

    // let m1_decomp = m1.lu_decomposition()?;
    // m1_decomp.show_matrix(f);
    // println!();

    // let tolerance = 1e-12f64;
    // let f = |data: &str| println!("{}", data);
    // let young_modulus = 3.0e7f64;
    // let poisson_ratio =  0.3f64;
    // let thickness =  0.15f64;

    // let integration_points = vec![
    //     (1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt()), (-1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt()),
    //     (-1.0 / 3f64.sqrt(), -1.0 / 3f64.sqrt()), (1.0 / 3f64.sqrt(), -1.0 / 3f64.sqrt()),
    // ];

    

    // let c_b_matrix_multiplier = young_modulus * thickness.powi(3) / (12f64 * (1f64 - poisson_ratio.powi(2)));
    // let mut c_b_matrix = ExtendedMatrix::create(
    //     3u32, 3u32, 
    //     vec![
    //         1f64, poisson_ratio, 0f64, 
    //         poisson_ratio, 1f64, 0f64,
    //         0f64, 0f64, (1f64 - poisson_ratio) / 2f64,
    //     ],
    //     tolerance)?;
    // c_b_matrix.multiply_by_number(c_b_matrix_multiplier);

    // let mut b_k = ExtendedMatrix::create(
    //     24u32, 24u32, vec![0f64; 24 * 24], tolerance)?;

    // for (r, s) in integration_points.iter()
    // {
    //     let b_k_i_values = vec![
    //         0.0, 0.0, 0.0, 0.0, -1.0 / 6.0 * (1.0 + s), 0.0,
    //         0.0, 0.0, 0.0, 0.0, 1.0 / 6.0 * (1.0 + s), 0.0,
    //         0.0, 0.0, 0.0, 0.0, 1.0 / 6.0 * (1.0 - s), 0.0,
    //         0.0, 0.0, 0.0, 0.0, -1.0 / 6.0 * (1.0 - s), 0.0,

    //         0.0, 0.0, 0.0, 1.0 / 4.0 * (1.0 + r), 0.0, 0.0,
    //         0.0, 0.0, 0.0, 1.0 / 4.0 * (1.0 - r), 0.0, 0.0,
    //         0.0, 0.0, 0.0, -1.0 / 4.0 * (1.0 - r), 0.0, 0.0,
    //         0.0, 0.0, 0.0, -1.0 / 4.0 * (1.0 + r), 0.0, 0.0,

    //         0.0, 0.0, 0.0, 1.0 / 6.0 * (1.0 + s), -1.0 / 4.0 * (1.0 + r), 0.0,
    //         0.0, 0.0, 0.0, -1.0 / 6.0 * (1.0 + s), -1.0 / 4.0 * (1.0 - r), 0.0,
    //         0.0, 0.0, 0.0, -1.0 / 6.0 * (1.0 - s), 1.0 / 4.0 * (1.0 - r), 0.0,
    //         0.0, 0.0, 0.0, 1.0 / 6.0 * (1.0 - s), 1.0 / 4.0 * (1.0 + r), 0.0,
    //     ];

    //     let b_k_i_matrix_rhs = ExtendedMatrix::create(
    //         3u32, 24u32, b_k_i_values, tolerance)?;
    //     let mut b_k_i_matrix_lhs = b_k_i_matrix_rhs.clone();
    //     b_k_i_matrix_lhs.transpose();

    //     let b_k_i_matrix = b_k_i_matrix_lhs.multiply_by_matrix(&c_b_matrix)?
    //         .multiply_by_matrix(&b_k_i_matrix_rhs)?;

    //     // b_k_i_matrix.show_matrix(f);

    //     b_k = b_k.add_matrix(&b_k_i_matrix)?;
    //     // println!();
    // }

    // // b_k.show_matrix(f);

    // // let integration_points = vec![
    // //     (0.0, 0.0),
    // // ];


    // let shear_factor = 0.8333;
    // let c_s_matrix_multiplier = young_modulus * thickness * shear_factor / (2f64 * (1f64 + poisson_ratio));
    // let mut c_s_matrix = ExtendedMatrix::create(
    //     2u32, 2u32, 
    //     vec![
    //         1f64, 0f64, 
    //         0f64, 1f64,
    //     ],
    //     tolerance)?;
    // c_s_matrix.multiply_by_number(c_s_matrix_multiplier);

    // let mut b_s = ExtendedMatrix::create(
    //     24u32, 24u32, vec![0f64; 24 * 24], tolerance)?;

    // for (r, s) in integration_points.iter()
    // {
    //     let b_s_i_values = vec![
    //         0.0, 0.0, 1.0 / 6.0 * (1.0 + s), 0.0, 1.0 / 4.0 * (1.0 + r) * (1.0 + s), 0.0,
    //         0.0, 0.0, -1.0 / 6.0 * (1.0 + s), 0.0, 1.0 / 4.0 * (1.0 - r) * (1.0 + s), 0.0,
    //         0.0, 0.0, -1.0 / 6.0 * (1.0 - s), 0.0, 1.0 / 4.0 * (1.0 - r) * (1.0 - s), 0.0,
    //         0.0, 0.0, 1.0 / 6.0 * (1.0 - s), 0.0, 1.0 / 4.0 * (1.0 + r) * (1.0 - s), 0.0,

    //         0.0, 0.0, 1.0 / 4.0 * (1.0 + r), -1.0 / 4.0 * (1.0 + r) * (1.0 + s), 0.0, 0.0,
    //         0.0, 0.0, 1.0 / 4.0 * (1.0 - r), -1.0 / 4.0 * (1.0 - r) * (1.0 + s), 0.0, 0.0,
    //         0.0, 0.0, -1.0 / 4.0 * (1.0 - r), -1.0 / 4.0 * (1.0 - r) * (1.0 - s), 0.0, 0.0,
    //         0.0, 0.0, -1.0 / 4.0 * (1.0 + r), -1.0 / 4.0 * (1.0 + r) * (1.0 - s), 0.0, 0.0,
    //     ];

    //     let b_s_i_matrix_rhs = ExtendedMatrix::create(
    //         2u32, 24u32, b_s_i_values, tolerance)?;
    //     let mut b_s_i_matrix_lhs = b_s_i_matrix_rhs.clone();
    //     b_s_i_matrix_lhs.transpose();

    //     let b_s_i_matrix = b_s_i_matrix_lhs.multiply_by_matrix(&c_s_matrix)?
    //         .multiply_by_matrix(&b_s_i_matrix_rhs)?;

    //     b_s_i_matrix.show_matrix(f);

    //     b_s = b_s.add_matrix(&b_s_i_matrix)?;
    //     println!();
    // }

    // b_s.show_matrix(f);

    // let tolerance = 1e-12f32;
    // let f = |data: &str| println!("{}", data);

    // let m_1 = ExtendedMatrix::create(512u32, 512u32, 
    //     vec![3.2f32; 262144], tolerance)?;
    // println!("m_1 was created: ");

    // let m_2 = ExtendedMatrix::create(512u32, 512u32, 
    //     vec![3.3f32; 262144], tolerance)?;
    // println!("m_2 was created: ");

    // let m_3 = m_1.multiply_by_matrix(&m_2)?;

    // println!("Result: ");

    // m_3.show_matrix(f);



    // let tolerance = 1e-12f32;
    // let f = |data: &str| println!("{}", data);

    // let heading = -45.0f32;
    // let bank = 35.264f32;

    // let sh = heading.to_radians().sin();
    // let ch = heading.to_radians().cos();

    // let sb = bank.to_radians().sin();
    // let cb = bank.to_radians().cos();

    // let r_h = ExtendedMatrix::create(3u32, 3u32, 
    //     vec![ch, 0.0, -sh, 0.0, 1.0, 0.0, -sh, 0.0, ch], tolerance)?;
        
    // let r_b = ExtendedMatrix::create(3u32, 3u32,
    //     vec![1.0, 0.0, 0.0, 0.0, cb, -sb, 0.0, sb, cb], tolerance)?;

    // let rotation_matrix = r_b.multiply_by_matrix(&r_h)?;

    // // let inv_rotation_matrix = rotation_matrix.inverse()?;

    // let up = ExtendedMatrix::create(1u32, 3u32, 
    //     vec![-1.3877787807814457e-17, 0.0197834226851167, 0.4921395613542412], tolerance)?;

    // let transformed_up = up.multiply_by_matrix(&rotation_matrix)?;

    // transformed_up.show_matrix(f);



    let f = |data: &str| println!("{}", data);

    let v_1 = Vector3::create(&[10.0, 0.0, 1.0]);
    let v_2 = Vector3::create(&[-10.0, 0.0, 1.0]);

    let rotation_matrix = v_1.rotation_matrix_to_align_with_vector(&v_2, 0.0001, 1e-6).unwrap();
    rotation_matrix.show(f);
    println!();

    let v_3 = rotation_matrix.transpose().multiply(&v_2).unwrap();
    v_3.show(f);
    println!();

    // let sm = SquareMatrix::create(3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 11.0]);
    // let sm = SquareMatrix::create(3, vec![2.0, 1.0, 0.0, 1.0, 3.0, 1.0, 0.0, 1.0, 2.0]);
    // let sm = SquareMatrix::create(2, vec![4.0, 2.0, 0.0, 6.0]);
    let sm = SquareMatrix::create(2, &[1.0, 1e4, 1e-4, 2.0]);
    let det = sm.determinant(0.0001).unwrap();
    println!("Determinant: {}", det);

    Ok(())
}


// fn show_matrix<M, V, F>(m: M, f: F)
//     where F: Fn(&str),
//           V: Copy + Debug,
//           M: BasicOperationsTrait<M, Value = V>
// {
//     let NewShape(rows_number, columns_numbers) = m.get_shape();
//     for row in 0..*rows_number
//     {
//         let mut row_str = String::from("[");
//         for column in 0..*columns_numbers
//         {
//             let pos = Position(row, column);
//             let value = m.get_element_value(&pos);
//             row_str += &format!("{:?}, ", value);
//         }
//         row_str = row_str[..row_str.len() - 2].to_string();
//         row_str += "]";
//         f(&format!("{}", row_str));
//     }
// }
