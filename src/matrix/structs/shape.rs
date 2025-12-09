#[derive(Debug, PartialEq, Clone)]
pub struct Shape(pub usize, pub usize);

impl Shape {
    pub fn update(&mut self, rows_number: usize, columns_number: usize) {
        (self.0, self.1) = (rows_number, columns_number);
    }

    pub fn swap_rows_number_and_columns_number(&mut self) {
        (self.0, self.1) = (self.1, self.0);
    }
}
