// external imports
use extended_matrix_float::MyFloatTrait;

use crate::{BasicOperationsTrait, FloatTrait, Operation, Position, Vector, VectorTrait};

fn pivot_gep<V, SMT, VT>(
    a: &mut SMT,
    b: &mut VT,
    s: &mut [<SMT as BasicOperationsTrait>::Value],
    n: usize,
    k: usize,
    pn: &mut i32,
) where
    V: FloatTrait,
    SMT: SquareMatrixTrait,
    VT: VectorTrait,
    <SMT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
    <VT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
{
    let mut p = k;
    let mut big = (*a
        .get_element_value(&Position(k, k))
        .expect("Element is absent")
        / s[k])
        .my_abs();
    for ii in k + 1..n {
        let dummy = (*a
            .get_element_value(&Position(ii, k))
            .expect("Element is absent")
            / s[ii])
            .my_abs();
        if dummy > big {
            big = dummy;
            p = ii;
        }
    }
    if p != k {
        *pn += 1;
        for jj in k..n {
            let dummy = *a
                .get_element_value(&Position(p, jj))
                .expect("Element is absent");
            *a.get_mut_element_value(&Position(p, jj))
                .expect("Element is absent") = *a
                .get_element_value(&Position(k, jj))
                .expect("Element is absent");
            *a.get_mut_element_value(&Position(k, jj))
                .expect("Element is absent") = dummy;
        }
        let dummy = *b
            .get_element_value(&Position(p, 0))
            .expect("Element is absent");
        *b.get_mut_element_value(&Position(p, 0))
            .expect("Element is absent") = *b
            .get_element_value(&Position(k, 0))
            .expect("Element is absent");
        *b.get_mut_element_value(&Position(k, 0))
            .expect("Element is absent") = dummy;
        let dummy = s[p];
        s[p] = s[k];
        s[k] = dummy;
    }
}

fn eliminate_gep<V, SMT, VT>(
    a: &mut SMT,
    s: &mut [<SMT as BasicOperationsTrait>::Value],
    n: usize,
    b: &mut VT,
    rel_tol: <SMT as BasicOperationsTrait>::Value,
) -> Result<i32, String>
where
    V: FloatTrait,
    SMT: SquareMatrixTrait + BasicOperationsTrait<Value = V>,
    VT: VectorTrait + BasicOperationsTrait<Value = V>,
    <SMT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
    <VT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
{
    let mut pn = 0i32;
    for k in 0..n - 1 {
        pivot_gep(a, b, s, n, k, &mut pn);
        if (*a
            .get_element_value(&Position(k, k))
            .expect("Element is absent")
            / s[k])
            .my_abs()
            < rel_tol
        {
            return Err("Ill conditioned system".to_string());
        }
        for i in k + 1..n {
            let factor = *a
                .get_element_value(&Position(i, k))
                .expect("Element is absent")
                / *a.get_element_value(&Position(k, k))
                    .expect("Element is absent");
            for j in k + 1..n {
                *a.get_mut_element_value(&Position(i, j))
                    .expect("Element is absent") = *a
                    .get_element_value(&Position(i, j))
                    .expect("Element is absent")
                    - factor
                        * *a.get_element_value(&Position(k, j))
                            .expect("Element is absent");
            }
            *b.get_mut_element_value(&Position(i, 0))
                .expect("Element is absent") = *b
                .get_element_value(&Position(i, 0))
                .expect("Element is absent")
                - factor
                    * *b.get_element_value(&Position(k, 0))
                        .expect("Element is absent");
        }
    }
    if (*a
        .get_element_value(&Position(n - 1, n - 1))
        .expect("Element is absent")
        / s[n - 1])
        .my_abs()
        < rel_tol
    {
        return Err("Ill conditioned system".to_string());
    }
    Ok(pn)
}

fn substitute_gep<V, SMT, VT1, VT2>(a: &SMT, n: usize, b: &VT1, x: &mut VT2)
where
    V: FloatTrait<Output = V>,
    SMT: SquareMatrixTrait + BasicOperationsTrait<Value = V>,
    VT1: VectorTrait + BasicOperationsTrait<Value = V>,
    VT2: VectorTrait + BasicOperationsTrait<Value = V>,
{
    *x.get_mut_element_value(&Position(n - 1, 0))
        .expect("Element is absent") = *b
        .get_element_value(&Position(n - 1, 0))
        .expect("Element is absent")
        / *a.get_element_value(&Position(n - 1, n - 1))
            .expect("Element is absent");
    for i in (0..n - 1).rev() {
        let mut sum = V::from(0f32);
        for j in i + 1..n {
            sum += *a
                .get_element_value(&Position(i, j))
                .expect("Element is absent")
                * *x.get_element_value(&Position(j, 0))
                    .expect("Element is absent");
        }
        *x.get_mut_element_value(&Position(i, 0))
            .expect("Element is absent") = (*b
            .get_element_value(&Position(i, 0))
            .expect("Element is absent")
            - sum)
            / *a.get_element_value(&Position(i, i))
                .expect("Element is absent");
    }
}

fn pivot_lup<V, SMT, VT>(
    a: &mut SMT,
    o: &mut [usize],
    s: &[<SMT as BasicOperationsTrait>::Value],
    n: usize,
    k: usize,
) where
    V: FloatTrait,
    SMT: SquareMatrixTrait,
    VT: VectorTrait,
    <SMT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
    <VT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
{
    let mut p = k;
    let mut big = (*a
        .get_element_value(&Position(o[k], k))
        .expect("Element is absent")
        / s[o[k]])
        .my_abs();
    for ii in k + 1..n {
        let dummy = (*a
            .get_element_value(&Position(o[ii], k))
            .expect("Element is absent")
            / s[o[ii]])
            .my_abs();
        if dummy > big {
            big = dummy;
            p = ii;
        }
    }
    let dummy = o[p];
    o[p] = o[k];
    o[k] = dummy;
}

fn decompose_lup<V, SMT, VT>(
    a: &mut SMT,
    n: usize,
    rel_tol: <SMT as BasicOperationsTrait>::Value,
    o: &mut [usize],
    s: &mut [<SMT as BasicOperationsTrait>::Value],
) -> Result<(), String>
where
    V: FloatTrait,
    SMT: SquareMatrixTrait + BasicOperationsTrait<Value = V>,
    VT: VectorTrait + BasicOperationsTrait<Value = V>,
    <SMT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
    <VT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
{
    for i in 0..n {
        o[i] = i;
        s[i] = (*a
            .get_element_value(&Position(i, 0))
            .expect("Element is absent"))
        .my_abs();
        for j in 1..n {
            if (*a
                .get_element_value(&Position(i, j))
                .expect("Element is absent"))
            .my_abs()
                > s[i]
            {
                s[i] = (*a
                    .get_element_value(&Position(i, j))
                    .expect("Element is absent"))
                .my_abs();
            }
        }
    }
    for k in 0..n - 1 {
        pivot_lup::<V, SMT, VT>(a, o, s, n, k);
        if (*a
            .get_element_value(&Position(o[k], k))
            .expect("Element is absent")
            / s[o[k]])
            .my_abs()
            < rel_tol
        {
            return Err("Ill conditioned system".to_string());
        }
        for i in k + 1..n {
            let factor = *a
                .get_element_value(&Position(o[i], k))
                .expect("Element is absent")
                / *a.get_element_value(&Position(o[k], k))
                    .expect("Element is absent");
            *a.get_mut_element_value(&Position(o[i], k))
                .expect("Element is absent") = factor;
            for j in k + 1..n {
                *a.get_mut_element_value(&Position(o[i], j))
                    .expect("Element is absent") = *a
                    .get_element_value(&Position(o[i], j))
                    .expect("Element is absent")
                    - factor
                        * *a.get_element_value(&Position(o[k], j))
                            .expect("Element is absent");
            }
        }
    }
    if (*a
        .get_element_value(&Position(o[n - 1], n - 1))
        .expect("Element is absent")
        / s[o[n - 1]])
        .my_abs()
        < rel_tol
    {
        return Err("Ill conditioned system".to_string());
    }
    Ok(())
}

fn substitute_lup<V, SMT, VT1, VT2>(a: &SMT, o: &[usize], n: usize, b: &mut VT1, x: &mut VT2)
where
    V: FloatTrait<Output = V>,
    SMT: SquareMatrixTrait + BasicOperationsTrait<Value = V>,
    VT1: VectorTrait + BasicOperationsTrait<Value = V>,
    VT2: VectorTrait + BasicOperationsTrait<Value = V>,
{
    for i in 1..n {
        let mut sum = *b
            .get_element_value(&Position(o[i], 0))
            .expect("Element is absent");
        for j in 0..i {
            sum -= *a
                .get_element_value(&Position(o[i], j))
                .expect("Element is absent")
                * *b.get_element_value(&Position(o[j], 0))
                    .expect("Element is absent");
        }
        *b.get_mut_element_value(&Position(o[i], 0))
            .expect("Element is absent") = sum;
    }
    *x.get_mut_element_value(&Position(n - 1, 0))
        .expect("Element is absent") = *b
        .get_element_value(&Position(o[n - 1], 0))
        .expect("Element is absent")
        / *a.get_element_value(&Position(o[n - 1], n - 1))
            .expect("Element is absent");
    for i in (0..n - 1).rev() {
        let mut sum = V::from(0f32);
        for j in i + 1..n {
            sum += *a
                .get_element_value(&Position(o[i], j))
                .expect("Element is absent")
                * *x.get_element_value(&Position(j, 0))
                    .expect("Element is absent");
        }
        *x.get_mut_element_value(&Position(i, 0))
            .expect("Element is absent") = (*b
            .get_element_value(&Position(o[i], 0))
            .expect("Element is absent")
            - sum)
            / *a.get_element_value(&Position(o[i], i))
                .expect("Element is absent");
    }
}

pub trait SquareMatrixTrait: BasicOperationsTrait {
    fn determinant(
        &self,
        rel_tol: <Self as BasicOperationsTrait>::Value,
    ) -> <Self as BasicOperationsTrait>::Value
    where
        Self: Clone,
        <Self as BasicOperationsTrait>::Value:
            FloatTrait<Output = <Self as BasicOperationsTrait>::Value>,
    {
        let mut a = self.clone();
        let n = a.get_shape().0;
        let mock_b_values = vec![<<Self as BasicOperationsTrait>::Value>::from(0f32); n];
        let mut mock_b = Vector::create(&mock_b_values);
        let mut s = vec![<<Self as BasicOperationsTrait>::Value>::from(0f32); n];
        for i in 0..n {
            s[i] = (*a
                .get_element_value(&Position(i, 0))
                .expect("Element is absent"))
            .my_abs();
            for j in 1..n {
                if (*a
                    .get_element_value(&Position(i, j))
                    .expect("Element is absent"))
                .my_abs()
                    > s[i]
                {
                    s[i] = (*a
                        .get_element_value(&Position(i, j))
                        .expect("Element is absent"))
                    .my_abs();
                }
            }
        }
        match eliminate_gep(&mut a, &mut s, n, &mut mock_b, rel_tol) {
            Ok(pn) => {
                let mut det = <<Self as BasicOperationsTrait>::Value>::from(1f32);
                for i in 0..a.get_shape().0 {
                    det *= *a
                        .get_element_value(&Position(i, i))
                        .expect("Element is absent");
                }
                det *= <<Self as BasicOperationsTrait>::Value>::from(-1f32).my_powi(pn);
                if det.my_is_nan() {
                    return <<Self as BasicOperationsTrait>::Value>::from(0f32);
                }
                det
            }
            Err(_) => <<Self as BasicOperationsTrait>::Value>::from(0f32),
        }
    }

    fn gauss_gep<VT1, VT2>(
        &self,
        b: &VT1,
        x: &mut VT2,
        rel_tol: <Self as BasicOperationsTrait>::Value,
    ) -> Result<(), String>
    where
        VT1: VectorTrait
            + BasicOperationsTrait<Value = <Self as BasicOperationsTrait>::Value>
            + Clone,
        VT2: VectorTrait
            + BasicOperationsTrait<Value = <Self as BasicOperationsTrait>::Value>
            + Clone,
        Self: Clone,
        <Self as BasicOperationsTrait>::Value:
            FloatTrait<Output = <Self as BasicOperationsTrait>::Value>,
    {
        let mut a = self.clone();
        let mut b = b.clone();
        b.vector_shape_conformity_check()?;
        x.vector_shape_conformity_check()?;
        if b.get_shape().0 == 1 {
            b = b.transpose();
        }
        if x.get_shape().0 == 1 {
            *x = x.transpose();
        }
        a.shape_conformity_check(&b, Operation::Multiplication)?;
        b.shape_conformity_check(x, Operation::Addition)?;

        let n = a.get_shape().0;
        let mut s = vec![<<Self as BasicOperationsTrait>::Value>::from(0f32); n];
        for i in 0..n {
            s[i] = (*a
                .get_element_value(&Position(i, 0))
                .expect("Element is absent"))
            .my_abs();
            for j in 1..n {
                if (*a
                    .get_element_value(&Position(i, j))
                    .expect("Element is absent"))
                .my_abs()
                    > s[i]
                {
                    s[i] = (*a
                        .get_element_value(&Position(i, j))
                        .expect("Element is absent"))
                    .my_abs();
                }
            }
        }
        let _ = eliminate_gep(&mut a, &mut s, n, &mut b, rel_tol)?;
        substitute_gep(&a, n, &b, x);
        Ok(())
    }

    fn inverse<VT>(
        &self,
        x: &mut VT,
        rel_tol: <Self as BasicOperationsTrait>::Value,
    ) -> Result<Self, String>
    where
        Self: Clone,
        <Self as BasicOperationsTrait>::Value:
            FloatTrait<Output = <Self as BasicOperationsTrait>::Value>,
        VT: VectorTrait
            + BasicOperationsTrait<Value = <Self as BasicOperationsTrait>::Value>
            + Clone,
    {
        let mut a_i = self.clone();
        for value in a_i.get_mut_elements().values_mut() {
            *value = <<Self as BasicOperationsTrait>::Value>::from(0f32);
        }

        let mut a = self.clone();
        let n = a.get_shape().0;
        let mut o = vec![0usize; n];
        let mut s = vec![<<Self as BasicOperationsTrait>::Value>::from(0f32); n];

        decompose_lup::<<Self as BasicOperationsTrait>::Value, Self, VT>(
            &mut a, n, rel_tol, &mut o, &mut s,
        )?;

        let b_values = vec![<<Self as BasicOperationsTrait>::Value>::from(0f32); n];
        let mut b = Vector::create(&b_values);

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    *b.get_mut_element_value(&Position(j, 0))
                        .expect("Element is absent") =
                        <<Self as BasicOperationsTrait>::Value>::from(1f32);
                } else {
                    *b.get_mut_element_value(&Position(j, 0))
                        .expect("Element is absent") =
                        <<Self as BasicOperationsTrait>::Value>::from(0f32);
                }
            }

            substitute_lup(&a, &o, n, &mut b, x);

            for j in 0..n {
                *a_i.get_mut_element_value(&Position(j, i))
                    .expect("Element is absent") = *x
                    .get_mut_element_value(&Position(j, 0))
                    .expect("Element is absent");
            }
        }

        Ok(a_i)
    }

    fn lup_decomp<VT1, VT2>(
        &self,
        b: &VT1,
        x: &mut VT2,
        rel_tol: <Self as BasicOperationsTrait>::Value,
    ) -> Result<(), String>
    where
        VT1: VectorTrait
            + BasicOperationsTrait<Value = <Self as BasicOperationsTrait>::Value>
            + Clone,
        VT2: VectorTrait
            + BasicOperationsTrait<Value = <Self as BasicOperationsTrait>::Value>
            + Clone,
        Self: Clone,
        <Self as BasicOperationsTrait>::Value:
            FloatTrait<Output = <Self as BasicOperationsTrait>::Value>,
    {
        let mut a = self.clone();
        let mut b = b.clone();
        b.vector_shape_conformity_check()?;
        x.vector_shape_conformity_check()?;
        if b.get_shape().0 == 1 {
            b = b.transpose();
        }
        if x.get_shape().0 == 1 {
            *x = x.transpose();
        }
        a.shape_conformity_check(&b, Operation::Multiplication)?;
        b.shape_conformity_check(x, Operation::Addition)?;

        let n = a.get_shape().0;
        let mut o = vec![0usize; n];
        let mut s = vec![<<Self as BasicOperationsTrait>::Value>::from(0f32); n];

        decompose_lup::<<Self as BasicOperationsTrait>::Value, Self, VT1>(
            &mut a, n, rel_tol, &mut o, &mut s,
        )?;
        substitute_lup(&a, &o, n, &mut b, x);

        Ok(())
    }
}
