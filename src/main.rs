use std::sync::Arc;

use camera::Camera;
use hittable::World;
use material::{Material, Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::Vec3;
use ray::Ray;
use utils::{random, random_range};


mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    let mut world: World = World::new();
    
    let camera: Camera = Camera::new(
        16.0 / 9.0,
	1200,
        100,
        50,
	20.0,
        0.6,
	10.0,
        Vec3::from_point(13.0, 2.0, 3.0),
        Vec3::from_point(0.0, 0.0, 0.0),
        Vec3::from_point(0.0, 1.0, 0.0),
    );

    let ground: Arc<Lambertian> = Arc::new(Lambertian::new(Vec3::from_point(0.5, 0.5, 0.5)));
    let glass: Arc<Dielectric> = Arc::new(Dielectric::new(1.5));

    world.push(Box::new(Sphere::stationary_from_dim(Vec3::from_point(0.0, -1000.0, 0.0), 1000.0, ground)));

    for a in -11..11 {
	for b in -11..11 {
	    let val = random();

	    let center = Vec3::from_point(a as f32 + 0.9 * val, 0.2, b as f32 + 0.9 * val);
	    let center_dir = Vec3::from_point(0.0, random() * 0.2, 0.0);
	    let center = Ray::construct(center, center_dir, 0.0);

	    if (center.origin - Vec3::from_point(4.0, 0.2, 0.0)).length() < 0.9 {
		continue
	    }
	    
	    let material_choice = random();
	    let mat: Arc<dyn Material> = if material_choice < 0.6 {
		// Lambertian
		let albedo = Vec3::random() * Vec3::random();
		Arc::new(Lambertian::new(albedo))
	    } else if material_choice < 0.9 {
		// Metal
		let albedo = Vec3::random_range(0.5..1.0);
		let fuzz = random_range(0.0..0.5);

		Arc::new(Metal::new(albedo, fuzz))
	    } else {
		// Dielectric
		glass.clone()
	    };

	    world.push(Box::new(Sphere::moving_from_dim(center, 0.2, mat)));
	}
    }

    world.push(Box::new(Sphere::stationary_from_dim(Vec3::from_point(0.0, 1.0, 0.0), 1.0, glass.clone())));
    world.push(Box::new(Sphere::stationary_from_dim(Vec3::from_point(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian::new(Vec3::from_point(0.4, 0.2, 0.1))))));
    world.push(Box::new(Sphere::stationary_from_dim(Vec3::from_point(4.0, 1.0, 0.0), 1.0, Arc::new(Metal::new(Vec3::from_point(1.0, 0.6, 0.6), 0.0)))));
    
    camera.render(&world);
}
