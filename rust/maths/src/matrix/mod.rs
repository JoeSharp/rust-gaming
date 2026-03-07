mod approx_eq;
mod classification;
mod core;
mod display;
mod operations;

#[macro_use]
mod matrix_macro;

#[derive(PartialEq, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub enum MatrixError {
    IncorrectDataSize,
    IncompatibleDimensions,
    SquareMatrixRequired,
    InvalidIndex(usize, usize),
}
impl Matrix {
    fn get_index(&self, row: usize, column: usize) -> Result<usize, MatrixError> {
        if row >= self.rows {
            return Err(MatrixError::InvalidIndex(row, column));
        }
        if column >= self.columns {
            return Err(MatrixError::InvalidIndex(row, column));
        }

        Ok(self.get_index_ok(row, column))
    }

    fn get_index_ok(&self, row: usize, column: usize) -> usize {
        column + (row * self.columns)
    }
}
