use std::ops::Mul;
use crate::vec2::Vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat2 {
    values: [f64; 4]
}

impl Mat2 {
    pub fn new(m11: f64, m12: f64, m21: f64, m22: f64) -> Self {
        Mat2 {
            values: [m11, m12, m21, m22]
        }
    }

    /// Creates a matrix that projects from the standard basis to the given basis vectors
    pub fn new_to_basis(x: Vector2<f64>, y: Vector2<f64>) -> Self {
        let from_basis = Mat2::new(x.x, y.x, x.y, y.y);
        from_basis.inverse().unwrap()
    }

    pub fn det(&self) -> f64 {
        (self.values[0] * self.values[3]) - (self.values[2] * self.values[1])
    }

    pub fn inverse(&self) -> Option<Mat2> {
        let determinant = self.det();
        if determinant.abs() < f64::EPSILON {
            return None;
        }

        Some(Mat2 {
            values: [
                self.values[3] / determinant,
                -self.values[1] / determinant,
                -self.values[2] / determinant,
                self.values[0] / determinant
            ]
        })
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, other: Mat2) -> Mat2 {
        Mat2::new(
            self.values[0] * other.values[0] + self.values[1] * other.values[2],
            self.values[0] * other.values[1] + self.values[1] * other.values[3],
            self.values[2] * other.values[0] + self.values[3] * other.values[2],
            self.values[2] * other.values[1] + self.values[3] * other.values[3]
        )
    }
}

impl Mul<Vector2<f64>> for Mat2 {
    type Output = Vector2<f64>;

    fn mul(self, other: Vector2<f64>) -> Vector2<f64> {
        Vector2::new(
            self.values[0] * other.x + self.values[1] * other.y,
            self.values[2] * other.x + self.values[3] * other.y
        )
    }
}

#[test]
fn mat2_test_det() {
    let mat = Mat2::new(1.0, 2.0, 3.0,4.0);
    assert_eq!(mat.det(), -2.0);

    let mat2 = Mat2::new(4.0, 6.0, 3.0, 8.0);
    assert_eq!(mat2.det(), 14.0);
}

#[test]
fn mat2_test_inverse() {
    let mat = Mat2::new(4.0, 7.0, 2.0, 6.0);
    assert_eq!(mat.inverse(), Some(Mat2::new(0.6, -0.7, -0.2, 0.4)));
}

#[test]
fn mat2_test_mul() {
    let mat = Mat2::new(1.0, 2.0, 3.0, 4.0);
    let mat2 = Mat2::new(2.0, 0.0, 1.0, 2.0);

    assert_eq!(mat * mat2, Mat2::new(4.0, 4.0, 10.0, 8.0));

    let v = Vector2::new(2.0, 1.0);
    assert_eq!(mat * v, Vector2::new(4.0, 10.0));
}

#[test]
fn mat2_test_projection() {
    // rotation
    let bx = Vector2::new(0.0, 1.0);
    let by = Vector2::new(-1.0, 0.0);
    let mat = Mat2::new_to_basis(bx, by);
    let v = Vector2::new(1.0, 1.0);

    assert_eq!(mat * v, Vector2::new(1.0, -1.0));
}