use crate::{BasicOperationsTrait, Shape, SquareMatrix};

pub trait TryIntoSquareMatrixTrait: BasicOperationsTrait {
    fn try_into_square_matrix(
        &self,
    ) -> Result<SquareMatrix<<Self as BasicOperationsTrait>::Value>, String>
    where
        <Self as BasicOperationsTrait>::Value: Copy,
    {
        let Shape(rows_number, columns_number) = self.get_shape();
        if *rows_number != *columns_number {
            return Err("Could not be converted into square matrix!".to_string());
        }

        Ok(SquareMatrix {
            shape: self.get_shape().clone(),
            elements: self.get_elements().clone(),
        })
    }
}
