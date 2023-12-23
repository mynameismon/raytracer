use crate::vec3::{Vec3, Point3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

#[allow(dead_code)]
impl Ray {
    pub fn new () -> Ray {
	Ray {
	    origin: Point3::new(),
	    direction: Vec3::new()
	}
    }

    pub fn construct (origin: Vec3, direction: Point3) -> Ray {
	Ray {
	    origin,
	    direction
	}
    }
}
