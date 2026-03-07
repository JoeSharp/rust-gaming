use super::Vector3;
use crate::approx_eq::ApproxEq;

impl ApproxEq for Vector3 {
    fn approx_eq(&self, other: &Vector3, eps: f64) -> bool {
        self.x.approx_eq(&other.x, eps)
            && self.y.approx_eq(&other.y, eps)
            && self.z.approx_eq(&other.z, eps)
    }
}
