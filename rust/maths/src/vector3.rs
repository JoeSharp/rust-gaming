use crate::approx_eq::ApproxEq;
use crate::matrix::Matrix;
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ApproxEq for Vector3 {
    fn approx_eq(&self, other: &Vector3, eps: f64) -> bool {
        self.x.approx_eq(&other.x, eps)
            && self.y.approx_eq(&other.y, eps)
            && self.z.approx_eq(&other.z, eps)
    }
}

impl Vector3 {
    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Vector3 {
        Vector3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn multiply(&self, multiplier: f64) -> Vector3 {
        Vector3 {
            x: self.x * multiplier,
            y: self.y * multiplier,
            z: self.z * multiplier,
        }
    }

    pub fn dot_product(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude(&self) -> f64 {
        let x_sq = self.x.powf(2.0);
        let y_sq = self.y.powf(2.0);
        let z_sq = self.z.powf(2.0);
        let sum_sq = x_sq + y_sq + z_sq;
        sum_sq.sqrt()
    }

    pub fn angle_between(&self, other: &Vector3) -> f64 {
        let dot = self.dot_product(other);
        let mags = self.magnitude() * other.magnitude();

        let cos_theta = dot / mags;
        cos_theta.acos()
    }

    pub fn normalize(&self) -> Vector3 {
        let mag = self.magnitude();
        println!("Mag {}", mag);
        Vector3::new(self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn cross_product(&self, other: &Vector3) -> Vector3 {
        let x = matrix!(
            rows: 2,
            cols: 2,
            self.y, self.z,
            other.y, other.z,
        )
        .determinant()
        .expect("det x");
        let y = -1.0
            * matrix!(
                rows: 2,
                cols: 2,
                self.x, self.z,
                other.x, other.z,
            )
            .determinant()
            .expect("det y");
        let z = matrix!(
            rows: 2,
            cols: 2,
            self.x, self.y,
            other.x, other.y,
        )
        .determinant()
        .expect("det z");
        Vector3 { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct VectorResultCase {
        a: Vector3,
        b: Vector3,
        expected: Vector3,
    }
    struct ScalarResultCase {
        a: Vector3,
        b: Vector3,
        expected: f64,
    }

    #[test]
    fn addition() {
        let cases: Vec<VectorResultCase> = vec![
            VectorResultCase {
                a: Vector3::new(3, 4, 3.2),
                b: Vector3::new(7, 2, 9.4),
                expected: Vector3::new(10, 6, 12.6),
            },
            VectorResultCase {
                a: Vector3::new(-2, 15, -2.5),
                b: Vector3::new(9, 2.1, 4),
                expected: Vector3::new(7, 17.1, 1.5),
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
                a: Vector3::new(3, 4, 3.2),
                b: Vector3::new(7, 2, 1.3),
                expected: Vector3::new(-4, 2, 1.9),
            },
            VectorResultCase {
                a: Vector3::new(-2, 15, -7),
                b: Vector3::new(9, 2.1, -4),
                expected: Vector3::new(-11, 12.9, -3),
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
            input: Vector3,
            multiplier: f64,
            expected: Vector3,
        }

        let cases: Vec<MultCase> = vec![MultCase {
            input: Vector3::new(5.4, 3.2, -4.1),
            multiplier: 4.0,
            expected: Vector3::new(21.6, 12.8, -16.4),
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
                a: Vector3::new(1, 0, 0),
                b: Vector3::new(0, 5, 0),
                expected: 0.0,
            },
            ScalarResultCase {
                a: Vector3::new(1, -2, 3),
                b: Vector3::new(4, 0.5, -1),
                expected: 0.0,
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
            a: Vector3::new(2, 2, -1),
            b: Vector3::new(5, -3, 2),
            expected: 0.108_f64.acos(),
        }];

        for case in cases {
            let result = case.a.angle_between(&case.b);

            assert!(result.approx_eq(&case.expected, 0.001));
        }
    }

    #[test]
    fn cross_product() {
        let cases: Vec<VectorResultCase> = vec![
            VectorResultCase {
                a: Vector3::new(1, 0, 0),
                b: Vector3::new(0, 1, 0),
                expected: Vector3::new(0, 0, 1),
            },
            VectorResultCase {
                a: Vector3::new(2, -1, 3),
                b: Vector3::new(0, 4, -2),
                expected: Vector3::new(-10, 4, 8),
            },
        ];

        for case in cases {
            let result = case.a.cross_product(&case.b);

            assert!(result.approx_eq_default(&case.expected));
        }
    }

    #[test]
    fn normalize() {
        struct NormalizeCase {
            input: Vector3,
            expected: Vector3,
        }

        let cases: Vec<NormalizeCase> = vec![
            NormalizeCase {
                input: Vector3::new(1, 2, 2),
                expected: Vector3::new(1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0),
            },
            NormalizeCase {
                input: Vector3::new(2, -3, 6),
                expected: Vector3::new(2.0 / 7.0, -3.0 / 7.0, 6.0 / 7.0),
            },
        ];

        for case in cases {
            let result = case.input.normalize();
            assert!(result.approx_eq_default(&case.expected));
        }
    }
}
