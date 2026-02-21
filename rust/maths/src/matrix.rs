#[allow(unused_imports)]
use crate::matrix;

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

    pub fn determinant(&self) -> Result<f64, MatrixError> {
        if self.rows != self.columns {
            return Err(MatrixError::SquareMatrixRequired);
        }

        if self.rows == 2 {
            return Ok(self.data[0] * self.data[3] - self.data[1] * self.data[2]);
        }

        let mut result = 0.0;

        for column_mask in 0..self.columns {
            let coeff_idx = self.get_index_ok(0, column_mask);
            let coeff_sign = if column_mask % 2 == 0 { 1.0 } else { -1.0 };
            let coeff = coeff_sign * self.data[coeff_idx];

            let mut sub_m = Matrix::square_zeros(self.rows - 1);
            for row in 1..self.rows {
                let mut column_index = 0;
                for column in 0..self.columns {
                    if column == column_mask {
                        continue;
                    }

                    let cell_value = self.get(row, column).expect("get to copy");
                    sub_m
                        .set(row - 1, column_index, cell_value)
                        .expect("set sub matrix item");
                    column_index += 1;
                }
            }

            result += coeff * sub_m.determinant().expect("Should know its square");
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn check_addition() {
        let m1 = matrix!(
            rows: 2,
            cols: 2,
            1.0, 2.0;
            3.0, 4.0
        );
        let m2 = matrix!(
            rows: 2,
            cols: 2,
            5.0, 6.0;
            7.0, 8.0
        );

        let result = m1.sum(&m2).expect("Addition should succeed");

        assert_eq!(result.get(0, 0).expect("r00"), 6.0);
        assert_eq!(result.get(0, 1).expect("r00"), 8.0);
        assert_eq!(result.get(1, 0).expect("r00"), 10.0);
        assert_eq!(result.get(1, 1).expect("r00"), 12.0);
    }

    #[test]
    fn check_multiplication() {
        let m1 = matrix!(
            rows: 2,
            cols: 3,
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0
        );
        let m2 = matrix!(
            rows: 3,
            cols: 2,
            7.0, 8.0;
            9.0, 10.0;
            11.0, 12.0
        );

        let result = m1.multiply(&m2).expect("Multiply should work");

        assert_eq!(result.get(0, 0).expect("r00"), 58.0);
        assert_eq!(result.get(0, 1).expect("r00"), 64.0);
        assert_eq!(result.get(1, 0).expect("r00"), 139.0);
        assert_eq!(result.get(1, 1).expect("r00"), 154.0);
    }

    #[test]
    fn multiplication_error() {
        let m1 = Matrix::zeros(3, 2);
        let m2 = Matrix::zeros(3, 2);

        m1.multiply(&m2).expect_err("Should be incompatible");
    }

    #[test]
    fn determinant() {
        struct Case {
            matrix: Matrix,
            expected: f64,
        }
        let cases = vec![
            Case {
                matrix: matrix!(
                    rows: 2,
                    cols: 2,
                    1.0, 2.0;
                    3.0, 4.0
                ),
                expected: -2.0,
            },
            Case {
                matrix: Matrix::identity(2),
                expected: 1.0,
            },
            Case {
                matrix: Matrix::identity(3),
                expected: 1.0,
            },
            Case {
                matrix: matrix!(
                    rows: 3,
                    cols: 3,
                    1.0, 2.0, 3.0;
                    4.0, 5.0, 6.0;
                    7.0, 8.0, 9.0
                ),
                expected: 0.0,
            },
            Case {
                matrix: matrix!(
                    rows: 3,
                    cols: 3,
                    -3.0, 2.0, -5.0;
                    -1.0, 0.0, -2.0;
                    3.0, -4.0, 1.0
                ),
                expected: -6.0,
            },
            Case {
                matrix: matrix!(
                    rows: 3,
                    cols: 3,
                    6.0, 1.0, 1.0;
                    4.0, -2.0, 5.0;
                    2.0, 8.0, 7.0
                ),
                expected: -306.0,
            },
            Case {
                matrix: matrix!(
                    rows: 4,
                    cols: 4,
                    1.0, 2.0, 3.0, 4.0;
                    5.0, 6.0, 7.0, 8.0;
                    9.0, 10.0, 11.0, 12.0;
                    2.0, 6.0, 4.0, 8.0
                ),
                expected: 0.0,
            },
            Case {
                matrix: matrix!(
                    rows: 4,
                    cols: 4,
                    3.0, 2.0, 0.0, 1.0;
                    4.0, 0.0, 1.0, 2.0;
                    3.0, 0.0, 2.0, 1.0;
                    9.0, 2.0, 3.0, 1.0
                ),
                expected: 24.0,
            },
        ];

        for (i, case) in cases.iter().enumerate() {
            let result = case.matrix.determinant().expect("should calc");

            assert_eq!(result, case.expected, "case {} failed", i);
        }
    }

    #[test]
    fn determinant_error() {
        let m1 = Matrix::zeros(2, 3);

        let result = m1.determinant().expect_err("Only works for square");

        assert_eq!(result, MatrixError::SquareMatrixRequired);
    }
}
