use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::ops::Range;
use std::sync::Arc;

#[allow(dead_code)]
/// Records any information associated with a hit: where did it hit, what is
/// normal to it. This is processed in every loop to actually "scatter" light.
/// Refer to: [Hittable], [material::Material]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Arc<dyn Material>,

    pub front: bool,
}

#[allow(dead_code)]
impl HitRecord {
    /// Creates a new instance of [HitRecord]. Assumes that the front face is true by default.
    pub fn new(point: Point3, normal: Vec3, t: f32, material: Arc<dyn Material>) -> Self {
        Self {
            point,
            normal,
            t,
            material,
            front: true,
        }
    }

    /// Depending on the current HitRecord, sets the normal in the correct outward direction.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front = r.direction.dot(outward_normal) < 0.0;

        self.normal = if self.front {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

/// Allows the creation of abstract shapes that can define the behaviour of reflection.
/// On hitting any surface, the ``hit()`` function will be invoked by the raytracer.
pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}

/// Used to hold the complete scene.
/// Largely, is a convienient wrapper for raytracing all the objects in the scene
pub type World = Vec<Box<dyn Hittable>>;


/// Raytraces all the objects in the scene.
/// It finds the closest value of ``t`` for which the ray is hit,
/// thus indirectly, checking the closest object hit.
impl Hittable for World {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let mut closest_yet: f32 = t_range.end;

        let mut hit: Option<HitRecord> = None;

        for object in self {
            if let Some(rec) = object.hit(r, t_range.start..closest_yet) {
                closest_yet = closest_yet.min(rec.t);
                hit = Some(rec)
            }
        }
        hit
    }
}
