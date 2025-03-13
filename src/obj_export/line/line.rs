use std::f64::consts::PI;

use crate::obj_export::vector3::vector3::Vector3;

#[derive(Debug, Clone)]
pub struct Line {
    pub points: [Vector3; 2],
    pub a: f64,
    pub b: f64,
}

impl Line {
    pub fn new() -> Self {
        Self {
            points: [Vector3::new(), Vector3::new()],
            a: 0.0,
            b: 0.0,
        }
    }

    pub fn from(points: [Vector3; 2]) -> Self {
        let a = (points[1].z - points[0].z) / (points[1].x - points[0].x);
        let b = points[0].z - (a * points[0].x);
        Self { points, a, b }
    }

    pub fn f(&self, x: f64) -> f64 {
        self.a * x + self.b
    }

    pub fn is_intesect(&self, other: &Self) -> bool {
        let x = self.intersect(other);
        !x.is_nan() && (x >= self.points[0].x && x <= self.points[1].x)
    }

    pub fn intersect(&self, other: &Self) -> f64 {
        (other.b - self.b) / (self.a - other.a)
    }

    pub fn angle(&self, other: &Self) -> f64 {
        let vec1 = (
            self.points[1].x - self.points[0].x,
            self.points[1].z - self.points[0].z,
        );

        let vec2 = (
            other.points[1].x - other.points[0].x,
            other.points[1].z - other.points[0].z,
        );

        let norm_1 = (vec1.0 * vec1.0 + vec1.1 * vec1.1).sqrt();
        let norm_2 = (vec2.0 * vec2.0 + vec2.1 * vec2.1).sqrt();
        180.0
            - ((vec1.0 * vec2.0 + vec1.1 * vec2.1) / (norm_2 * norm_1)).acos()
                * (180.0 / PI)
    }
}
