use super::Matrix;
use crate::approx_eq::ApproxEq;

impl Matrix {
    pub fn is_square(&self) -> bool {
        self.rows == self.columns
    }

    pub fn is_upper_triangular(&self) -> bool {
        for row in 0..self.rows {
            let limit = if row > self.columns {
                self.columns
            } else {
                row
            };
            for col in 0..limit {
                match self.get(row, col) {
                    Ok(v) => {
                        if !v.approx_eq_default(&0.0) {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
        }

        true
    }

    pub fn is_lower_triangular(&self) -> bool {
        for row in 0..self.rows {
            for col in (row + 1)..self.columns {
                match self.get(row, col) {
                    Ok(v) => {
                        if !v.approx_eq_default(&0.0) {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FormCase {
        m: Matrix,
        is_upper_triangular: bool,
        is_lower_triangular: bool,
    }

    fn form_cases() -> Vec<FormCase> {
        vec![
            FormCase {
                m: matrix!(
                    rows: 2,
                    cols: 2,
                    1, 1;
                    0, 1
                ),
                is_upper_triangular: true,
                is_lower_triangular: false,
            },
            FormCase {
                m: matrix!(
                    rows: 3,
                    cols: 3,
                    1, 2, 3,
                    0, 4, 5,
                    0, 0, 6
                ),
                is_upper_triangular: true,
                is_lower_triangular: false,
            },
            FormCase {
                m: matrix!(
                    rows: 3,
                    cols: 3,
                    1, 0, 0,
                    2, 3, 0,
                    4, 5, 6
                ),
                is_upper_triangular: false,
                is_lower_triangular: true,
            },
            FormCase {
                m: matrix!(
                    rows: 3,
                    cols: 3,
                    1, 0, 1,
                    2, 3, 0,
                    4, 5, 6
                ),
                is_upper_triangular: false,
                is_lower_triangular: false,
            },
            FormCase {
                m: Matrix::identity(3),
                is_upper_triangular: true,
                is_lower_triangular: true,
            },
            FormCase {
                m: matrix!(
                    rows: 3,
                    cols: 3,
                    1, 2, 3;
                    4, 5, 6;
                    7, 8, 9
                ),
                is_upper_triangular: false,
                is_lower_triangular: false,
            },
            FormCase {
                m: matrix!(
                    rows: 3,
                    cols: 5,
                    1, 2, 3, 4, 5;
                    0, 6, 7, 8, 9;
                    0, 0, 1, 2, 3
                ),
                is_upper_triangular: true,
                is_lower_triangular: false,
            },
            FormCase {
                m: matrix!(
                    rows: 5,
                    cols: 3,
                    1, 2, 3;
                    0, 4, 5;
                    0, 0, 6;
                    0, 0, 0;
                    0, 0, 0
                ),
                is_upper_triangular: true,
                is_lower_triangular: false,
            },
            FormCase {
                m: matrix!(
                    rows: 3,
                    cols: 5,
                    1, 2, 3, 4, 5;
                    0, 6, 7, 8, 9;
                    1, 0, 1, 2, 3
                ),
                is_upper_triangular: false,
                is_lower_triangular: false,
            },
            FormCase {
                m: matrix!(
                    rows: 5,
                    cols: 3,
                    1, 2, 3;
                    0, 4, 5;
                    0, 0, 6;
                    0, 0, 0;
                    0, 0, 1
                ),
                is_upper_triangular: false,
                is_lower_triangular: false,
            },
            FormCase {
                m: matrix!(
                    rows: 3,
                    cols: 5,
                    1, 0, 0, 0, 0;
                    2, 6, 0, 0, 0;
                    5, 3, 1, 0, 0
                ),
                is_upper_triangular: false,
                is_lower_triangular: true,
            },
            FormCase {
                m: matrix!(
                    rows: 5,
                    cols: 3,
                    1, 0, 0;
                    4, 4, 0;
                    4, 4, 6;
                    4, 4, 4;
                    4, 4, 4
                ),
                is_upper_triangular: false,
                is_lower_triangular: true,
            },
            FormCase {
                m: matrix!(
                    rows: 3,
                    cols: 5,
                    1, 0, 0, 0, 1;
                    2, 6, 0, 0, 0;
                    5, 3, 1, 0, 0
                ),
                is_upper_triangular: false,
                is_lower_triangular: false,
            },
            FormCase {
                m: matrix!(
                    rows: 5,
                    cols: 3,
                    1, 0, 1;
                    4, 4, 0;
                    4, 4, 6;
                    4, 4, 4;
                    4, 4, 4
                ),
                is_upper_triangular: false,
                is_lower_triangular: false,
            },
        ]
    }

    #[test]
    fn is_upper_triangular() {
        for case in form_cases() {
            let result = case.m.is_upper_triangular();

            assert_eq!(
                result, case.is_upper_triangular,
                "{} should be upper triangular",
                case.m
            );
        }
    }

    #[test]
    fn is_lower_triangular() {
        for case in form_cases() {
            let result = case.m.is_lower_triangular();

            assert_eq!(
                result, case.is_lower_triangular,
                "{} should be lower triangular",
                case.m
            );
        }
    }
}
