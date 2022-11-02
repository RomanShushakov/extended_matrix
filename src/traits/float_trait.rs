use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign};
use std::fmt::Debug;

use extended_matrix_float::MyFloatTrait;


pub trait FloatTrait: 
    Debug +
    Copy +
    PartialEq +
    PartialOrd +
    AddAssign +
    SubAssign +
    MulAssign +
    From<f32> +
    Into<f64> +
    Add<Output = <Self as FloatTrait>::Output> +
    Sub<Output = <Self as FloatTrait>::Output> +
    Mul<Output = <Self as FloatTrait>::Output> +
    Div<Output = <Self as FloatTrait>::Output> +
    MyFloatTrait +
    'static
{ 
    type Output;
}


impl FloatTrait for f32 
{
    type Output = f32;
}


impl FloatTrait for f64
{
    type Output = f64;
}
