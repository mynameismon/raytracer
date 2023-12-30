use rayon::prelude::*;

use crate::hittable::{Hittable, World};
use crate::ray::Ray;
use crate::utils::{random, random_range};
use crate::vec3::{Point3, Vec3};
use core::f32::INFINITY;

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f32,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,

    image_height: u32,
    camera_center: Point3,
    init_pixel_loc: Point3,

    delta_u: Vec3,
    delta_v: Vec3,
    defocus_u: Vec3,
    defocus_v: Vec3
}

type Colour = Vec3;

#[allow(dead_code)]
impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f32,
	defocus_angle: f32,
	focus_distance: f32,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
	    defocus_angle,
	    focus_distance,
            image_height: 0,
            camera_center: Vec3::new(),
            init_pixel_loc: Vec3::new(),
            delta_u: Vec3::new(),
            delta_v: Vec3::new(),
	    defocus_u: Vec3::new(),
	    defocus_v: Vec3::new()
        }
        .initialize()
    }

    fn initialize(mut self) -> Self {
        // Camera setup. Currently at the origin
        self.camera_center = self.lookfrom;

        // TODO: We currently ignore case when height is < 1
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        let viewport_ratio: f32 = self.image_width as f32 / self.image_height as f32;

        let height: f32 = (self.vfov.to_radians() / 2.0).tan();
        let viewport_height: f32 = 2.0 * height * self.focus_distance;
        let viewport_width: f32 = viewport_height * viewport_ratio;

	// Set orthonormal viewport vectors
        let w = (self.lookfrom - self.lookat).unit();
        let u = self.vup.cross(w).unit();
        let v = w.cross(u);

        // Viewport vectors. Since the pixels are numbered from the top,
        // while the camera has axis facing upwards, we are forced to mark
        // the v vector with negative direction
        let viewport_u: Vec3 = viewport_width * u;
        let viewport_v: Vec3 = viewport_height * -v;

        self.delta_u = viewport_u / (self.image_width as f32);
        self.delta_v = viewport_v / (self.image_height as f32);

        let viewport_topleft = self.camera_center
            - ((self.focus_distance * w) + viewport_u / 2.0 + viewport_v / 2.0);

        self.init_pixel_loc = viewport_topleft + 0.5 * (self.delta_u + self.delta_v);

	let defocus_radius = self.focus_distance * (self.defocus_angle.to_radians() / 2.0).tan();
	self.defocus_u = defocus_radius * u;
	self.defocus_v = defocus_radius * v;

        self
    }

    fn pixel_sample_square(&self) -> Vec3 {
        0.5 * random() * self.delta_u + 0.5 * random() * self.delta_v
    }

    fn sample_disc (&self) -> Vec3 {
	loop {
	    let x = random_range(-1.0..1.0);
	    let y = random_range(-1.0..1.0);

	    if (x.powi(2) + y.powi(2)) < 1.0 {
		return self.camera_center + x * self.defocus_u + y * self.defocus_v
	    }
	}
    }
    
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.init_pixel_loc
            + (i as f32) * self.delta_u
            + (j as f32) * self.delta_v
            + self.pixel_sample_square();

	let ray_origin = if self.defocus_angle <= 0.0 {
	    self.camera_center
	} else {
	    self.sample_disc()
	};
	
        Ray::construct(ray_origin, pixel_center - ray_origin)
    }

    pub fn render(&self, world: &World) {
        self.debug();

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rNumber of lines remaining: {}", self.image_height - j);

            for i in 0..self.image_width {
                let pixel: Vec3 = (0..self.samples_per_pixel)
                    .into_par_iter()
                    .map(|_| ray_colour(&self.get_ray(i, j), world, self.max_depth))
                    .reduce(|| Vec3::new(), |sum, x| sum + x)
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
