#[derive(PartialEq, Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
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
                a: Vector3 {
                    x: 3.0,
                    y: 4.0,
                    z: 3.2,
                },
                b: Vector3 {
                    x: 7.0,
                    y: 2.0,
                    z: 9.4,
                },
                expected: Vector3 {
                    x: 10.0,
                    y: 6.0,
                    z: 12.6,
                },
            },
            VectorResultCase {
                a: Vector3 {
                    x: -2.0,
                    y: 15.0,
                    z: -2.5,
                },
                b: Vector3 {
                    x: 9.0,
                    y: 2.1,
                    z: 4.0,
                },
                expected: Vector3 {
                    x: 7.0,
                    y: 17.1,
                    z: 1.5,
                },
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
                a: Vector3 {
                    x: 3.0,
                    y: 4.0,
                    z: 3.2,
                },
                b: Vector3 {
                    x: 7.0,
                    y: 2.0,
                    z: 1.3,
                },
                expected: Vector3 {
                    x: -4.0,
                    y: 2.0,
                    z: 1.9,
                },
            },
            VectorResultCase {
                a: Vector3 {
                    x: -2.0,
                    y: 15.0,
                    z: -7.0,
                },
                b: Vector3 {
                    x: 9.0,
                    y: 2.1,
                    z: -4.0,
                },
                expected: Vector3 {
                    x: -11.0,
                    y: 12.9,
                    z: -3.0,
                },
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
            input: Vector3 {
                x: 5.4,
                y: 3.2,
                z: -4.1,
            },
            multiplier: 4.0,
            expected: Vector3 {
                x: 21.6,
                y: 12.8,
                z: -16.4,
            },
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
                a: Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                b: Vector3 {
                    x: 0.0,
                    y: 5.0,
                    z: 0.0,
                },
                expected: 0.0,
            },
            ScalarResultCase {
                a: Vector3 {
                    x: 1.0,
                    y: -2.0,
                    z: 3.0,
                },
                b: Vector3 {
                    x: 4.0,
                    y: 0.5,
                    z: -1.0,
                },
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
            a: Vector3 {
                x: 2.0,
                y: 2.0,
                z: -1.0,
            },
            b: Vector3 {
                x: 5.0,
                y: -3.0,
                z: 2.0,
            },
            expected: 0.108_f64.acos(),
        }];

        for case in cases {
            let result = case.a.angle_between(&case.b);

            approx_eq!(result, case.expected, 0.001);
        }
    }
}
