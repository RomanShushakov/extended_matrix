#[derive(Debug, PartialEq)]
pub struct Shape<T>(pub T, pub T);


#[derive(Debug, PartialEq, Clone)]
pub struct NewShape(pub usize, pub usize);


impl NewShape
{
    pub fn update(&mut self, rows_number: usize, columns_number: usize)
    {
        (self.0, self.1) = (rows_number, columns_number);
    }


    pub fn swap_rows_number_and_columns_number(&mut self)
    {
        (self.0, self.1) = (self.1, self.0);
    }
}
