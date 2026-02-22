use crate::approx_eq::ApproxEq;

#[derive(PartialEq, Debug)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl ApproxEq for Vector2 {
    fn approx_eq(&self, other: &Vector2, eps: f64) -> bool {
        self.x.approx_eq(&other.x, eps) && self.y.approx_eq(&other.y, eps)
    }
}

impl Vector2 {
    pub fn new<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Vector2 {
        Vector2 {
            x: x.into(),
            y: y.into(),
        }
    }
    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn subtract(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn multiply(&self, multiplier: f64) -> Vector2 {
        Vector2 {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
    }

    pub fn dot_product(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f64 {
        let x_sq = self.x.powf(2.0);
        let y_sq = self.y.powf(2.0);
        let sum_sq = x_sq + y_sq;
        sum_sq.sqrt()
    }

    pub fn angle_between(&self, other: &Vector2) -> f64 {
        let dot = self.dot_product(other);
        let mags = self.magnitude() * other.magnitude();

        let cos_theta = dot / mags;
        cos_theta.acos()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    struct VectorResultCase {
        a: Vector2,
        b: Vector2,
        expected: Vector2,
    }
    struct ScalarResultCase {
        a: Vector2,
        b: Vector2,
        expected: f64,
    }

    #[test]
    fn addition() {
        let cases: Vec<VectorResultCase> = vec![
            VectorResultCase {
                a: Vector2::new(3, 4),
                b: Vector2::new(7, 2),
                expected: Vector2::new(10, 6),
            },
            VectorResultCase {
                a: Vector2::new(-2, 15),
                b: Vector2::new(9, 2.1),
                expected: Vector2::new(7, 17.1),
            },
        ];

        for case in cases {
            let result = case.a.add(&case.b);

            assert!(result.approx_eq_default(&case.expected));
        }
    }

    #[test]
    fn subtraction() {
        let cases: Vec<VectorResultCase> = vec![
            VectorResultCase {
                a: Vector2::new(3, 4),
                b: Vector2::new(7, 2),
                expected: Vector2::new(-4, 2),
            },
            VectorResultCase {
                a: Vector2::new(-2, 15),
                b: Vector2::new(9, 2.1),
                expected: Vector2::new(-11, 12.9),
            },
        ];

        for case in cases {
            let result = case.a.subtract(&case.b);

            assert!(result.approx_eq_default(&case.expected));
        }
    }

    #[test]
    fn multiply() {
        struct MultCase {
            input: Vector2,
            multiplier: f64,
            expected: Vector2,
        }

        let cases: Vec<MultCase> = vec![MultCase {
            input: Vector2::new(5.4, 3.2),
            multiplier: 4.0,
            expected: Vector2::new(21.6, 12.8),
        }];

        for case in cases {
            let result = case.input.multiply(case.multiplier);

            assert!(result.approx_eq_default(&case.expected));
        }
    }

    #[test]
    fn dot_product() {
        let cases: Vec<ScalarResultCase> = vec![
            ScalarResultCase {
                a: Vector2::new(2, 7),
                b: Vector2::new(8.5, 3.1),
                expected: 38.7,
            },
            ScalarResultCase {
                a: Vector2::new(2, 4),
                b: Vector2::new(1, -3),
                expected: -10.0,
            },
        ];

        for case in cases {
            let result = case.a.dot_product(&case.b);

            assert!(result.approx_eq_default(&case.expected));
        }
    }

    #[test]
    fn angle_between() {
        let cases: Vec<ScalarResultCase> = vec![ScalarResultCase {
            a: Vector2::new(1, 0),
            b: Vector2::new(0, 1),
            expected: PI / 2.0,
        }];

        for case in cases {
            let result = case.a.angle_between(&case.b);

            assert!(result.approx_eq_default(&case.expected));
        }
    }
}
