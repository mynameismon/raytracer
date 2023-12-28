use std::sync::Arc;

use camera::Camera;
use hittable::World;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::Vec3;
// use material::{Metal, Lambertian};

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    let camera: Camera = Camera::new(16.0 / 9.0, 500, 100, 20);

    let ground: Arc<Lambertian> = Arc::new(Lambertian::new(Vec3::from_point(0.8, 0.8, 0.0)));
    let center: Arc<Lambertian> = Arc::new(Lambertian::new(Vec3::from_point(0.7, 0.3, 0.3)));
    let left: Arc<Metal> = Arc::new(Metal::new(Vec3::from_point(0.8, 0.8, 0.8), 0.2));
    let right: Arc<Dielectric> = Arc::new(Dielectric::new(3.0));

    let mut world: World = World::new();
    world.push(Box::new(Sphere::from_dim(
        Vec3::from_point(0.0, -100.5, -1.0),
        100.0,
        ground,
    )));

    world.push(Box::new(Sphere::from_dim(
        Vec3::from_point(0.0, 0.0, -1.0),
        0.5,
        center,
    )));
    world.push(Box::new(Sphere::from_dim(
        Vec3::from_point(-1.0, 0.0, -1.0),
        0.5,
        left,
    )));
    world.push(Box::new(Sphere::from_dim(
        Vec3::from_point(1.0, 0.0, -1.0),
        0.5,
        right,
    )));

    camera.render(&world);
}
