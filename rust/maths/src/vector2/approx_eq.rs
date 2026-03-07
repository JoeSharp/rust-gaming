use super::Vector2;
use crate::approx_eq::ApproxEq;

impl ApproxEq for Vector2 {
    fn approx_eq(&self, other: &Vector2, eps: f64) -> bool {
        self.x.approx_eq(&other.x, eps) && self.y.approx_eq(&other.y, eps)
    }
}
