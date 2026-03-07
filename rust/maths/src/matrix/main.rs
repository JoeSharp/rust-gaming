use super::{Matrix, MatrixError};

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

    pub fn identity(rows: usize) -> Self {
        let mut m = Matrix::square_zeros(rows);

        for i in 0..rows {
            let index = m.get_index_ok(i, i);
            m.data[index] = 1.0;
        }

        return m;
    }

    pub fn zeros(rows: usize, columns: usize) -> Self {
        return Matrix {
            rows,
            columns,
            data: vec![0.0; rows * columns],
        };
    }

    pub fn square_zeros(dimension: usize) -> Self {
        return Matrix {
            rows: dimension,
            columns: dimension,
            data: vec![0.0; dimension * dimension],
        };
    }

    // Joe - make this ... more private?
    pub fn get_index(&self, row: usize, column: usize) -> Result<usize, MatrixError> {
        if row >= self.rows {
            return Err(MatrixError::InvalidIndex(row, column));
        }
        if column >= self.columns {
            return Err(MatrixError::InvalidIndex(row, column));
        }

        Ok(self.get_index_ok(row, column))
    }

    // Joe - make this ... more private?
    pub fn get_index_ok(&self, row: usize, column: usize) -> usize {
        column + (row * self.columns)
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

    #[test]
    fn create_valid_matrix() {
        let m = Matrix::new(
            2,
            2,
            vec![
                1.0, 2.0, // row
                3.0, 4.0,
            ],
        );

        assert!(m.is_ok());
    }

    #[test]
    fn identity() {
        let m = Matrix::identity(3);

        assert_eq!(m.get(0, 0).unwrap(), 1.0);
        assert_eq!(m.get(1, 1).unwrap(), 1.0);
        assert_eq!(m.get(2, 2).unwrap(), 1.0);

        assert_eq!(m.get(0, 1).unwrap(), 0.0);
        assert_eq!(m.get(1, 2).unwrap(), 0.0);
        assert_eq!(m.get(2, 0).unwrap(), 0.0);
    }

    #[test]
    fn get_and_set_matrix_elements() {
        let mut m = matrix!(
            rows: 3,
            cols: 3,
            4.0, 8.0, 3.0;
            2.1, 5.6, 9.8;
            3.9, 9.3, 5.4
        );

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
        let mut m = Matrix::zeros(2, 2);

        m.get(3, 0).unwrap_err();
        m.set(2, 5, 6.7).unwrap_err();
    }

    #[test]
    fn create_invalid_matrix() {
        let data = vec![1.0, 2.0, 3.0];
        let m = Matrix::new(2, 2, data);

        assert!(m.is_err());
    }

    use super::*;
}
