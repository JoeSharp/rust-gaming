#[derive(PartialEq, Debug)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
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
                a: Vector2 { x: 3.0, y: 4.0 },
                b: Vector2 { x: 7.0, y: 2.0 },
                expected: Vector2 { x: 10.0, y: 6.0 },
            },
            VectorResultCase {
                a: Vector2 { x: -2.0, y: 15.0 },
                b: Vector2 { x: 9.0, y: 2.1 },
                expected: Vector2 { x: 7.0, y: 17.1 },
            },
        ];

        for case in cases {
            let result = case.a.add(&case.b);

            approx_vec2!(result, case.expected);
        }
    }

    #[test]
    fn subtraction() {
        let cases: Vec<VectorResultCase> = vec![
            VectorResultCase {
                a: Vector2 { x: 3.0, y: 4.0 },
                b: Vector2 { x: 7.0, y: 2.0 },
                expected: Vector2 { x: -4.0, y: 2.0 },
            },
            VectorResultCase {
                a: Vector2 { x: -2.0, y: 15.0 },
                b: Vector2 { x: 9.0, y: 2.1 },
                expected: Vector2 { x: -11.0, y: 12.9 },
            },
        ];

        for case in cases {
            let result = case.a.subtract(&case.b);

            approx_vec2!(result, case.expected);
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
            input: Vector2 { x: 5.4, y: 3.2 },
            multiplier: 4.0,
            expected: Vector2 { x: 21.6, y: 12.8 },
        }];

        for case in cases {
            let result = case.input.multiply(case.multiplier);

            approx_vec2!(result, case.expected);
        }
    }

    #[test]
    fn dot_product() {
        let cases: Vec<ScalarResultCase> = vec![
            ScalarResultCase {
                a: Vector2 { x: 2.0, y: 7.0 },
                b: Vector2 { x: 8.5, y: 3.1 },
                expected: 38.7,
            },
            ScalarResultCase {
                a: Vector2 { x: 2.0, y: 4.0 },
                b: Vector2 { x: 1.0, y: -3.0 },
                expected: -10.0,
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
            a: Vector2 { x: 1.0, y: 0.0 },
            b: Vector2 { x: 0.0, y: 1.0 },
            expected: PI / 2.0,
        }];

        for case in cases {
            let result = case.a.angle_between(&case.b);

            approx_eq!(result, case.expected);
        }
    }
}
