use extended_matrix_float::MyFloatTrait;

use crate::{BasicOperationsTrait, VectorTrait, Position, FloatTrait, Vector};


pub trait SquareMatrixTrait: BasicOperationsTrait 
{
    fn determinant(&self, rel_tol: <Self as BasicOperationsTrait>::Value) 
        -> Result<<Self as BasicOperationsTrait>::Value, String>
        where Self: Clone,
              <Self as BasicOperationsTrait>::Value: FloatTrait<Output = <Self as BasicOperationsTrait>::Value>,
    {
        let mut a = self.clone();
        let n = a.get_shape().0;
        let mock_b_values = 
            vec![<<Self as BasicOperationsTrait>::Value>::from(0f32); a.get_shape().0];
        let mut mock_b = Vector::create(&mock_b_values);
        let mut s = 
            vec![<<Self as BasicOperationsTrait>::Value>::from(0f32); a.get_shape().0];
        for i in 0..n
        {
            s[i] = (*a.get_element_value(&Position(i, 0)).expect("Element is absent")).my_abs();
            for j in 1..n
            {
                if (*a.get_element_value(&Position(i, j)).expect("Element is absent")).my_abs() > s[i]
                {
                    s[i] = (*a.get_element_value(&Position(i, j)).expect("Element is absent")).my_abs();
                }
            }
        };
        match eliminate_gep(&mut a, &mut s, n, &mut mock_b, rel_tol)
        {
            Ok(pn) => 
            {
                let mut det = <<Self as BasicOperationsTrait>::Value>::from(1f32);
                for i in 0..a.get_shape().0
                {
                    det *= *a.get_element_value(&Position(i, i)).expect("Element is absent");
                }
                det *= <<Self as BasicOperationsTrait>::Value>::from(-1f32).my_powi(pn);
                if det.my_is_nan()
                {
                    return Ok(<<Self as BasicOperationsTrait>::Value>::from(0f32));
                }
                Ok(det)
            },
            Err(_) => Ok(<<Self as BasicOperationsTrait>::Value>::from(0f32))
        }
    }
}


fn pivot_gep<V, SMT, VT>(a: &mut SMT, b: &mut VT, s: &mut [<SMT as BasicOperationsTrait>::Value], n: usize, k: usize, 
        pn: &mut i32)
    where V: FloatTrait,
          SMT: SquareMatrixTrait,
          VT: VectorTrait,
          <SMT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
          <VT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
{
    let mut p = k;
    let mut big = (*a.get_element_value(&Position(k, k))
        .expect("Element is absent") / s[k]).my_abs();
    for ii in k + 1..n
    {
        let dummy = (*a.get_element_value(&Position(ii, k))
            .expect("Element is absent") / s[ii]).my_abs();
        if dummy > big
        {
            big = dummy;
            p = ii;
        }
    }
    if p != k
    {
        *pn += 1;
        for jj in k..n
        {
            let dummy = *a.get_element_value(&Position(p, jj))
                .expect("Element is absent");
            *a.get_mut_element_value(&Position(p, jj)).expect("Element is absent") = 
                *a.get_element_value(&Position(k, jj)).expect("Element is absent");
            *a.get_mut_element_value(&Position(k, jj)).expect("Element is absent") = dummy;
        }
        let dummy = *b.get_element_value(&Position(p, 0))
            .expect("Element is absent");
        *b.get_mut_element_value(&Position(p, 0)).expect("Element is absent") = 
            *b.get_element_value(&Position(k, 0)).expect("Element is absent");
        *b.get_mut_element_value(&Position(k, 0)).expect("Element is absent") = dummy;
        let dummy = s[p];
        s[p] = s[k];
        s[k] = dummy;
    }
}


fn eliminate_gep<V, SMT, VT>(a: &mut SMT, s: &mut [<SMT as BasicOperationsTrait>::Value], n: usize, b: &mut VT, 
    rel_tol: <SMT as BasicOperationsTrait>::Value) -> Result<i32, String>
    where V: FloatTrait,
          SMT: SquareMatrixTrait + BasicOperationsTrait<Value = V>,
          VT: VectorTrait + BasicOperationsTrait<Value = V>,
          <SMT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
          <VT as BasicOperationsTrait>::Value: FloatTrait<Output = V>,
{
    let mut pn = 0i32;
    for k in 0..n - 1
    {
        pivot_gep(a, b, s, n, k, &mut pn);
        if (*a.get_element_value(&Position(k, k)).expect("Element is absent") / s[k]).my_abs() < rel_tol
        {
            return Err("Ill conditioned system".to_string());
        }
        for i in k + 1..n
        {
            let factor = *a.get_element_value(&Position(i, k)).expect("Element is absent") /
                *a.get_element_value(&Position(k, k)).expect("Element is absent");
            for j in k + 1..n
            {
                *a.get_mut_element_value(&Position(i, j)).expect("Element is absent") = 
                    *a.get_element_value(&Position(i, j)).expect("Element is absent") - factor * 
                    *a.get_element_value(&Position(k, j)).expect("Element is absent");
            }
            *b.get_mut_element_value(&Position(k, 0)).expect("Element is absent") = 
                *b.get_element_value(&Position(k, 0)).expect("Element is absent") - factor * 
                *b.get_element_value(&Position(k, 0)).expect("Element is absent");
        }
    }
    if (*a.get_element_value(&Position(n - 1, n - 1)).expect("Element is absent") / s[n - 1]).my_abs() < rel_tol
    {
        return Err("Ill conditioned system".to_string());
    }
    Ok(pn)
}
