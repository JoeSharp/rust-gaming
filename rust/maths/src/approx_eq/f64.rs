use super::ApproxEq;

impl ApproxEq for f64 {
    fn approx_eq(&self, b: &f64, eps: f64) -> bool {
        if (self - b).abs() > eps {
            println!("{} - {} = {}", self, b, self - b);
            return false;
        }

        true
    }
}
