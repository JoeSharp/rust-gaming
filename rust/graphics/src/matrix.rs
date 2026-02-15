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

        Ok(self.get_index_ok(row, column))
    }

    fn get_index_ok(&self, row: usize, column: usize) -> usize {
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

    pub fn sum(&self, other: &Matrix) -> Result<Matrix, MatrixError> {
        if self.rows != other.rows || self.columns != other.columns {
            return Err(MatrixError::IncompatibleDimensions);
        }

        let mut result = Matrix::zeros(self.rows, self.columns);
        for index in 0..self.data.len() {
            result.data[index] = self.data[index] + other.data[index];
        }
        Ok(result)
    }

    pub fn multiply(&self, other: &Matrix) -> Result<Matrix, MatrixError> {
        if self.rows != other.columns || self.columns != other.rows {
            return Err(MatrixError::IncompatibleDimensions);
        }

        let mut result = Matrix::zeros(self.rows, other.columns);
        for row in 0..self.rows {
            for column in 0..other.columns {
                let mut r = 0.0;
                for elem in 0..self.columns {
                    let s_index = self.get_index_ok(row, elem);
                    let o_index = other.get_index_ok(elem, column);
                    r = r + (self.data[s_index] * other.data[o_index]);
                }

                let index = result.get_index_ok(row, column);
                result.data[index] = r;
            }
        }

        Ok(result)
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

    #[test]
    fn check_addition() {
        let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).expect("e");
        let m2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]).expect("e");

        let result = m1.sum(&m2).expect("Addition should succeed");

        assert_eq!(result.get(0, 0).expect("r00"), 6.0);
        assert_eq!(result.get(0, 1).expect("r00"), 8.0);
        assert_eq!(result.get(1, 0).expect("r00"), 10.0);
        assert_eq!(result.get(1, 1).expect("r00"), 12.0);
    }

    #[test]
    fn check_multiplication() {
        let m1 = Matrix::new(
            2,
            3,
            vec![
                1.0, 2.0, 3.0, // row
                4.0, 5.0, 6.0, // row
            ],
        )
        .expect("m1");
        let m2 = Matrix::new(
            3,
            2,
            vec![
                7.0, 8.0, // row
                9.0, 10.0, // row
                11.0, 12.0, // row
            ],
        )
        .expect("m2");

        let result = m1.multiply(&m2).expect("Multiply should work");

        assert_eq!(result.get(0, 0).expect("r00"), 58.0);
        assert_eq!(result.get(0, 1).expect("r00"), 64.0);
        assert_eq!(result.get(1, 0).expect("r00"), 139.0);
        assert_eq!(result.get(1, 1).expect("r00"), 154.0);
    }
}
