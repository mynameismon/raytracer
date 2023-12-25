use hittable:: World;
use vec3::Vec3;
use sphere::Sphere;
use camera::Camera;

mod vec3;
mod ray;
mod sphere;
mod hittable;
mod camera;

fn main() {
    let camera: Camera = Camera::new(16.0/9.0, 1000, 10);
    
    let mut world: World = World::new();
    world.push(Box::new(Sphere::from_dim(Vec3::from_point(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::from_dim(Vec3::from_point(0.0, -100.5, -1.0), 100.0)));

    camera.render(&world);
}
