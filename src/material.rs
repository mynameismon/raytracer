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

	let unit = r_in.direction.unit();
	
        let cos_theta = (-1.0 * unit).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

	let cannot_refract = (sin_theta * refractive_index) > 1.0;
	let will_reflect = reflectance(cos_theta, refractive_index) > random();
	
        let direction = if cannot_refract || will_reflect {
            unit.reflect_along(rec.normal)
        } else {
            unit.refract_along(rec.normal, refractive_index)
        };

        Some(Reflect {
            attenuation: Vec3::from_point(1.0, 1.0, 1.0),
            scattered: Ray::construct(rec.point, direction),
        })
    }
}

fn reflectance(cos_theta: f32, ri: f32) -> f32 {
    let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
