use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use std::ops::Range;
use std::sync::Arc;

/// A sphere is represented very simply using a center, a radius and a material.
pub struct Sphere {
    pub center: Ray,
    pub radius: f32,
    pub material: Arc<dyn Material>
}

#[allow(dead_code)]
impl Sphere {
    /// Creates a new sphere from the given dimensions
    pub const fn stationary_from_dim (center: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::construct(center, Vec3::new(), 0.0),
            radius,
            material,
        }
    }

    pub const fn moving_from_dim (center: Ray, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material
        }
    }
}

// TODO: Implement a moving sphere. This requires keeping track of the time at which the intersectinos take place

impl Hittable for Sphere {
    /// A ray intersects a sphere only if ||(A + Bt)|| <= r^2
    /// We can resolve this as a quadratic equation in terms of t,
    /// that can be solved easily using the quadratic equation.
    ///
    /// Here, the ray direction represents Bt, while A = center - ray origin
    /// 
    /// It is important to set the outward normal as this will define what direction
    /// will the ray continue in. If the outward normal is not defined correctly,
    /// it would cause refracted rays to also return in the direction of the incident
    /// ray, causing major problems.
    #[allow(non_snake_case)]
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
	let center = self.center.at(r.time);
	
        let A: Vec3 = r.origin - center;
        let B: Vec3 = r.direction;

        let a = B.length_sq();
        let half_b = dot(A, B);
        let c = A.length_sq() - (self.radius * self.radius);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let disc_sqrt = discriminant.sqrt();
        let neg_root = (-half_b - disc_sqrt) / a;
        let pos_root = (-half_b + disc_sqrt) / a;

        let mut root = None;

        if t_range.contains(&neg_root) {
            root = Some(neg_root);
        } else if t_range.contains(&pos_root) {
            root = Some(pos_root);
        }

        root.map(|x| {
            let outward_normal = (r.at(x) - center) / self.radius;

            let mut rec = HitRecord::new(
                r.at(x),
                (r.at(x) - center) / self.radius,
                x,
                self.material.clone(),
            );

            rec.set_face_normal(r, outward_normal);
            rec
        })
    }
}
