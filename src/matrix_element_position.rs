use std::ops::SubAssign;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct MatrixElementPosition<T>
{
    row: T,
    column: T,
}


impl<T> MatrixElementPosition<T>
    where T: Copy + From<u8> + SubAssign
{
    pub fn create(row: T, column: T) -> Self
    {
        MatrixElementPosition { row, column }
    }


    pub fn ref_row(&self) -> &T
    {
        &self.row
    }


    pub fn ref_column(&self) -> &T
    {
        &self.column
    }


    pub(super) fn swap_row_and_column(&mut self)
    {
        let interim = self.row;
        self.row = self.column;
        self.column = interim;
    }


    pub(super) fn decrease_row_number(&mut self)
    {
        self.row -= T::from(1u8);
    }


    pub(super) fn decrease_column_number(&mut self)
    {
        self.column -= T::from(1u8);
    }
}
