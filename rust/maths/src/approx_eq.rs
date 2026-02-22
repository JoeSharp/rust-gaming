pub trait ApproxEq {
    fn approx_eq(&self, other: &Self, eps: f64) -> bool;

    fn approx_eq_default(&self, other: &Self) -> bool {
        self.approx_eq(other, 1e-6)
    }
}

impl ApproxEq for f64 {
    fn approx_eq(&self, b: &f64, eps: f64) -> bool {
        (self - b).abs() <= eps
    }
}
