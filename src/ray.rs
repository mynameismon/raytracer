use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub const fn new() -> Ray {
        Ray {
            origin: Point3::new(),
            direction: Vec3::new(),
        }
    }

    pub const fn construct(origin: Vec3, direction: Point3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
