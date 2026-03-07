use super::{Matrix, MatrixError};

impl Matrix {
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
