use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::ops::Range;
use std::sync::Arc;

#[allow(dead_code)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Arc<dyn Material>,

    pub front: bool,
}

#[allow(dead_code)]
impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f32, material: Arc<dyn Material>) -> Self {
        Self {
            point,
            normal,
            t,
            material,
            front: true,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front = r.direction.dot(outward_normal) < 0.0;

        self.normal = if self.front {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}

pub type World = Vec<Box<dyn Hittable>>;

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
