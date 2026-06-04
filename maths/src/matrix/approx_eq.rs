use super::Matrix;
use crate::approx_eq::ApproxEq;

impl ApproxEq for Matrix {
    fn approx_eq(&self, other: &Self, eps: f64) -> bool {
        if self.rows != other.rows {
            return false;
        }
        if self.columns != other.columns {
            return false;
        }

        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(a, b)| a.approx_eq(b, eps))
    }
}
