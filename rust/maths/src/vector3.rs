#[derive(PartialEq, Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
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

    pub fn cross_product(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
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

            approx_vec3!(result, case.expected);
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

            approx_vec3!(result, case.expected);
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

            approx_vec3!(result, case.expected);
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

            approx_eq!(result, case.expected);
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

            approx_eq!(result, case.expected, 0.001);
        }
    }

    //#[test]
    fn cross_product() {
        let cases: Vec<VectorResultCase> = vec![
            VectorResultCase {
                a: Vector3::new(1, 0, 0),
                b: Vector3::new(0, 1, 0),
                expected: Vector3::new(0, 0, 1),
            },
            VectorResultCase {
                a: Vector3::new(2, -1, 3),
                b: Vector3::new(0, 4, -1),
                expected: Vector3::new(-10, 4, 8),
            },
        ];

        for case in cases {
            let result = case.a.cross_product(&case.b);

            approx_vec3!(result, case.expected);
        }
    }
}
