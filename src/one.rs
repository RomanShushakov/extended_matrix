pub trait One
{
    fn one() -> Self;
}


impl One for u8
{
    fn one() -> Self
    {
        1u8
    }
}


impl One for u16
{
    fn one() -> Self
    {
        1u16
    }
}


impl One for u32
{
    fn one() -> Self
    {
        1u32
    }
}


impl One for u64
{
    fn one() -> Self
    {
        1u64
    }
}


impl One for usize
{
    fn one() -> Self
    {
        1usize
    }
}


impl One for f32
{
    fn one() -> Self
    {
        1f32
    }
}


impl One for f64
{
    fn one() -> Self
    {
        1f64
    }
}
