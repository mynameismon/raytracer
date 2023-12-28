use crate::hittable::{Hittable, World};
use crate::ray::Ray;
use crate::utils::random;
use crate::vec3::{Point3, Vec3};
use core::f32::INFINITY;

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    image_height: u32,
    camera_center: Point3,
    init_pixel_loc: Point3,

    delta_u: Vec3,
    delta_v: Vec3,
}

type Colour = Vec3;

#[allow(dead_code)]
impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: 0,
            camera_center: Vec3::new(),
            init_pixel_loc: Vec3::new(),
            delta_u: Vec3::new(),
            delta_v: Vec3::new(),
        }
        .initialize()
    }

    fn initialize(mut self) -> Self {
        // Camera setup. Currently at the origin
        self.camera_center = Vec3::new();
        let focal_length: f32 = 1.0;

        // TODO: We currently ignore case when height is < 1
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        let viewport_ratio: f32 = self.image_width as f32 / self.image_height as f32;

        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * viewport_ratio;

        // Viewport vectors. Since the pixels are numbered from the top,
        // while the camera has axis facing upwards, we are forced to mark
        // the v vector with negative direction
        let viewport_u: Vec3 = Vec3::from_point(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = -Vec3::from_point(0.0, viewport_height, 0.0);

        self.delta_u = viewport_u / (self.image_width as f32);
        self.delta_v = viewport_v / (self.image_height as f32);

        let viewport_topleft = self.camera_center
            - (Vec3::from_point(0.0, 0.0, focal_length) + viewport_u / 2.0 + viewport_v / 2.0);

        self.init_pixel_loc = viewport_topleft + 0.5 * (self.delta_u + self.delta_v);

        self
    }

    fn pixel_sample_square(&self) -> Vec3 {
        0.5 * random() * self.delta_u + 0.5 * random() * self.delta_v
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.init_pixel_loc
            + (i as f32) * self.delta_u
            + (j as f32) * self.delta_v
            + self.pixel_sample_square();

        Ray::construct(self.camera_center, pixel_center - self.camera_center)
    }

    pub fn render(&self, world: &World) {
        self.debug();

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rNumber of lines remaining: {}", self.image_height - j);

            for i in 0..self.image_width {
                let pixel = (0..self.samples_per_pixel)
                    .map(|_| ray_colour(&self.get_ray(i, j), world, self.max_depth))
                    .fold(Vec3::new(), |sum, x| sum + x)
                    / self.samples_per_pixel as f32;

                println!("{}", pixel);
            }
        }

        eprintln!();
        eprintln!("Done.");
    }

    pub fn debug(&self) {
        eprintln!("Image properties:");
        eprintln!("\tHeight: {}", self.image_height);
        eprintln!("\tWidth: {}", self.image_width);
        eprintln!("\tAspect Ratio: {}", self.aspect_ratio);

        eprintln!("Debug Info:");
        eprintln!("\tPixel width: {}", self.delta_u);
        eprintln!("\tPixel height: {}", self.delta_v);
        eprintln!("\tCamera Position: {}", self.camera_center);
    }
}

fn ray_colour(r: &Ray, world: &World, depth: u32) -> Colour {
    if depth == 0 {
        return Vec3::new();
    }

    match world.hit(r, 0.001..INFINITY) {
        Some(t) => {
            if let Some(x) = t.material.scatter(r, &t) {
                x.attenuation * ray_colour(&x.scattered, world, depth - 1)
            } else {
                Vec3::new()
            }
        }
        None => {
            let dir = r.direction.unit();
            let a = 0.5 * (dir.y + 1.0);

            (1.0 - a) * Vec3::from_point(1.0, 1.0, 1.0) + a * Vec3::from_point(0.5, 0.7, 1.0)
        }
    }
}
