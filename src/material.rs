use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::random;
use crate::vec3::{random_unit_vector, Point3, Vec3};

pub struct Reflect {
    pub attenuation: Point3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Reflect>;
}

pub struct Lambertian {
    albedo: Point3,
}

impl Lambertian {
    pub fn new(albedo: Point3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Reflect> {
        let scatter_dir = if (rec.normal + random_unit_vector()).near_zero() {
            rec.normal
        } else {
            rec.normal + random_unit_vector()
        };

        Some(Reflect {
            scattered: Ray::construct(rec.point, scatter_dir),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Point3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Point3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Reflect> {
        let reflected = r_in.direction.unit().reflect_along(rec.normal);

        let scattered_dir = reflected + self.fuzz * random_unit_vector();

        match scattered_dir.dot(rec.normal) > 0.0 {
            true => Some(Reflect {
                scattered: Ray::construct(rec.point, scattered_dir),
                attenuation: self.albedo,
            }),
            false => None,
        }
    }
}

pub struct Dielectric {
    pub eta: f32,
}

impl Dielectric {
    pub fn new(eta: f32) -> Self {
        Self { eta }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Reflect> {
        let refractive_index = if rec.front { 1.0 / self.eta } else { self.eta };

        let cos_theta = r_in.direction.unit().dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if sin_theta * refractive_index > 1.0
            || refractance(cos_theta, refractive_index) > random()
        {
            r_in.direction.reflect_along(rec.normal)
        } else {
            r_in.direction
                .unit()
                .refract_along(rec.normal, refractive_index)
        };

        Some(Reflect {
            attenuation: Vec3::from_point(1.0, 1.0, 1.0),
            scattered: Ray::construct(rec.point, direction),
        })
    }
}

fn refractance(cos_theta: f32, ri: f32) -> f32 {
    let r0 = (1.0 - ri) / (1.0 + ri);
    let r = r0 * r0;

    r + (1.0 - r) * (1.0 - cos_theta).powf(5.0)
}
