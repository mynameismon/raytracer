//! Simple abstraction to store a ray, that is defined by a point relative to the origin.

use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32
}

#[allow(dead_code)]
impl Ray {
    /// Creates a new ray at origin of length 0
    pub const fn new() -> Ray {
        Ray {
            origin: Point3::new(),
            direction: Vec3::new(),
	    time: 0.0
        }
    }

    /// Constructs a new ray from the point and direction
    pub const fn construct(origin: Vec3, direction: Point3, time: f32) -> Ray {
        Ray { origin, direction, time}
    }

    /// Calculates the point at ``t`` distance from the origin of the ray
    /// along the direction of the ray
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
