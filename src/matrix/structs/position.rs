#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub(crate) fn swap_row_and_column(&mut self) {
        (self.0, self.1) = (self.1, self.0);
    }
}
