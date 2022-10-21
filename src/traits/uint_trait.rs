use std::ops::{Add, Sub, Mul, Div, Rem, AddAssign, SubAssign};
use std::hash::Hash;
use std::fmt::Debug;


pub trait UIntTrait: 
    Debug +
    Copy + 
    PartialOrd +
    Eq +
    Ord +
    Hash +
    AddAssign +
    SubAssign +
    From<u8> +
    Add<Output = <Self as UIntTrait>::Output> +
    Sub<Output = <Self as UIntTrait>::Output> +
    Mul<Output = <Self as UIntTrait>::Output> +
    Div<Output = <Self as UIntTrait>::Output> +
    Rem<Output = <Self as UIntTrait>::Output> +
    'static
{
    type Output;
}


impl UIntTrait for u8
{
    type Output = u8;
}


impl UIntTrait for u16
{
    type Output = u16;
}


impl UIntTrait for u32
{
    type Output = u32;
}


impl UIntTrait for u64
{
    type Output = u64;
}


impl UIntTrait for usize
{
    type Output = usize;
}
