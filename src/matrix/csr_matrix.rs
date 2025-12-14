use crate::{BasicOperationsTrait, FloatTrait, Position, SquareMatrix};

#[derive(Clone, Debug)]
pub struct CsrMatrix<V> {
    n_rows: usize,
    n_cols: usize,
    values: Vec<V>,
    col_index: Vec<usize>,
    row_ptr: Vec<usize>, // length = n_rows + 1
}

impl<V> CsrMatrix<V>
where
    V: FloatTrait<Output = V> + Clone,
{
    pub fn create(
        n_rows: usize,
        n_cols: usize,
        values: Vec<V>,
        col_index: Vec<usize>,
        row_ptr: Vec<usize>,
    ) -> Result<Self, String> {
        if row_ptr.len() != n_rows + 1 {
            return Err("CsrMatrix::new: row_ptr length must be n_rows + 1".to_string());
        }
        if values.len() != col_index.len() {
            return Err("CsrMatrix::new: values and col_index length mismatch".to_string());
        }
        if *row_ptr.last().unwrap_or(&0) != values.len() {
            return Err("CsrMatrix::new: last row_ptr must equal values.len()".to_string());
        }

        Ok(Self {
            n_rows,
            n_cols,
            values,
            col_index,
            row_ptr,
        })
    }

    pub fn get_n_rows(&self) -> usize {
        self.n_rows
    }

    pub fn get_n_cols(&self) -> usize {
        self.n_cols
    }

    pub fn get_values(&self) -> &[V] {
        &self.values
    }

    pub fn get_col_index(&self) -> &[usize] {
        &self.col_index
    }

    pub fn get_row_ptr(&self) -> &[usize] {
        &self.row_ptr
    }

    pub fn from_square_matrix(a: &SquareMatrix<V>) -> Result<Self, String> {
        let a_shape = a.get_shape();
        let (n_rows, n_cols) = (a_shape.0, a_shape.1);

        if n_rows != n_cols {
            return Err("CsrMatrix::from_square_matrix: matrix is not square".to_string());
        }

        // Collect (row, col, value) triplets from the internal storage
        let elements = a.get_elements(); // likely &HashMap<Position, V>

        let mut triplets: Vec<(usize, usize, V)> = Vec::with_capacity(elements.len());
        for (pos, val) in elements.iter() {
            let Position(i, j) = *pos;
            // Skip exact zeros if they can appear:
            if *val == V::from(0.0_f32) {
                continue;
            }
            triplets.push((i, j, val.clone()));
        }

        // Sort by (row, col) to build CSR cleanly
        triplets.sort_by(|(i1, j1, _), (i2, j2, _)| i1.cmp(i2).then(j1.cmp(j2)));

        let nnz = triplets.len();
        let mut values = Vec::with_capacity(nnz);
        let mut col_index = Vec::with_capacity(nnz);
        let mut row_ptr = vec![0usize; n_rows + 1];

        let mut current_row = 0usize;
        let mut count_in_row = 0usize;

        for (i, j, v) in triplets.into_iter() {
            // Fill row_ptr for skipped empty rows
            while current_row < i {
                row_ptr[current_row + 1] = row_ptr[current_row] + count_in_row;
                current_row += 1;
                count_in_row = 0;
            }

            values.push(v);
            col_index.push(j);
            count_in_row += 1;
        }

        // Close remaining rows
        while current_row < n_rows {
            row_ptr[current_row + 1] = row_ptr[current_row] + count_in_row;
            current_row += 1;
            count_in_row = 0;
        }

        CsrMatrix::create(n_rows, n_cols, values, col_index, row_ptr)
    }

    pub fn spmv(&self, x: &[V]) -> Result<Vec<V>, String> {
        if x.len() != self.n_cols {
            return Err(format!(
                "CsrMatrix::spmv: dimension mismatch: A is {}x{}, x has len {}",
                self.n_rows,
                self.n_cols,
                x.len()
            ));
        }

        let mut y = vec![V::from(0.0_f32); self.n_rows];

        for i in 0..self.n_rows {
            let row_start = self.row_ptr[i];
            let row_end = self.row_ptr[i + 1];

            let mut sum = V::from(0.0_f32);

            for idx in row_start..row_end {
                let j = self.col_index[idx];
                let a_ij = &self.values[idx];
                let x_j = &x[j];
                sum = sum + (*a_ij) * (*x_j);
            }

            y[i] = sum;
        }

        Ok(y)
    }
}
