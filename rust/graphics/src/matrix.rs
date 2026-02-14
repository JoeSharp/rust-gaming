#[derive(PartialEq, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub enum MatrixError {
    IncorrectDataSize,
    InvalidIndex(usize, usize),
}

impl Matrix {
    pub fn new(rows: usize, columns: usize, data: Vec<f64>) -> Result<Self, MatrixError> {
        let expected_size = rows * columns;

        if data.len() != expected_size {
            return Err(MatrixError::IncorrectDataSize);
        }

        Ok(Matrix {
            rows,
            columns,
            data,
        })
    }

    pub fn zeros(rows: usize, columns: usize) -> Self {
        return Matrix {
            rows,
            columns,
            data: vec![0.0; rows * columns],
        };
    }

    fn get_index(&self, row: usize, column: usize) -> Result<usize, MatrixError> {
        if row >= self.rows {
            return Err(MatrixError::InvalidIndex(row, column));
        }
        if column >= self.columns {
            return Err(MatrixError::InvalidIndex(row, column));
        }
        return Ok(column + (row * self.columns));
    }

    pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
        return match self.get_index(row, column) {
            Ok(v) => Ok(self.data[v]),
            Err(e) => return Err(e),
        };
    }

    pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
        return match self.get_index(row, column) {
            Ok(v) => {
                self.data[v] = value;
                return Ok(());
            }
            Err(e) => Err(e),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_matrix() {
        let data = vec![
            1.0, 2.0, // row 1
            3.0, 4.0, // row 2
        ];
        let m = Matrix::new(2, 2, data);

        assert!(m.is_ok());
    }

    #[test]
    fn get_and_set_matrix_elements() {
        let data = vec![
            4.0, 8.0, 3.0, // row
            2.1, 5.6, 9.8, // row
            3.9, 9.3, 5.4,
        ];
        let mut m = Matrix::new(3, 3, data).expect("Matrix should succeed");

        let at12 = m.get(1, 2).expect("Index should be valid");
        let at20 = m.get(2, 0).expect("Index should be valid");
        m.set(1, 2, 4.78).expect("Set should work");
        let at12_after = m.get(1, 2).unwrap();

        assert_eq!(at12, 9.8);
        assert_eq!(at20, 3.9);
        assert_eq!(at12_after, 4.78);
    }

    #[test]
    fn get_set_out_of_bounds() {
        let data = vec![
            1.3, 5.6, // row
            7.4, 9.2, // row
        ];
        let mut m = Matrix::new(2, 2, data).expect("Matrix should create");

        m.get(3, 0).unwrap_err();
        m.set(2, 5, 6.7).unwrap_err();
    }

    #[test]
    fn create_invalid_matrix() {
        let data = vec![1.0, 2.0, 3.0];
        let m = Matrix::new(2, 2, data);

        assert!(m.is_err());
    }
}
