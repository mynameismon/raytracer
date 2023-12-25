use crate::ray::Ray;
use crate::vec3::{Vec3, Point3, dot};
use crate::hittable::{Hittable, HitRecord};
use std::ops::Range;

#[derive(Debug)]
pub struct Sphere {
    pub centre: Point3,
    pub radius: f32
}

#[allow(dead_code)]
impl Sphere {
    pub const fn new () -> Self {
	Self {
	    centre: Point3::new(),
	    radius: 0.0
	}
    }

    pub const fn from_dim (centre: Point3, radius: f32) -> Self {
	Self {
	    centre,
	    radius
	}
    }
}

impl Hittable for Sphere {
    // Idea: A ray intersects a sphere only if ||(A + Bt)|| <= r^2
    // We can resolve this as a quadratic equation in terms of t,
    // that can be solved easily using the quadratic equation.
    //
    // Here, the ray direction represents Bt, while A = center - ray origin
    #[allow(non_snake_case)]
    fn hit (&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
	let A: Vec3 = r.origin - self.centre;
	let B: Vec3 = r.direction;
	
	let a = B.norm();
	let half_b = dot(A, B);
	let c = A.norm() - (self.radius * self.radius);
	
	let discriminant = half_b * half_b -  a * c;
	if discriminant < 0.0 {
	    return None
	}

	let disc_sqrt = discriminant.sqrt();
	let neg_root = (-half_b - disc_sqrt) / a;
	let pos_root = (- half_b + disc_sqrt) / a;

	let mut root = None;
	
	if t_range.contains(&neg_root) {
	    root = Some(neg_root);
	} else if t_range.contains(&pos_root) {
	    root = Some(pos_root);
	}

	match root {
	    Some(x) => {
		Some (HitRecord::new(r.at(x),(r.at(x) - self.centre) / self.radius, x))
	    },
	    None => None
	}
    }
}
