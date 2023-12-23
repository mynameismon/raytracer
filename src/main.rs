use vec3::{Vec3, Point3};
use ray::Ray;

mod vec3;
mod ray;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

type Colour = Vec3;

#[allow(dead_code)]
fn ray_colour (x: &Ray) -> Colour {
    let dir = x.direction.unit();
    let a = 0.5 * (dir.y + 1.0);

    (1.0 - a) * Vec3::from_point(1.0, 1.0, 1.0) + a * Vec3::from_point(0.5, 0.7, 1.0)
}

fn main() {
    eprintln!("Starting output print...");

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 400;

    // TODO: Currently ignore case when height is < 1
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    let viewport_ratio = image_height as f32 / image_width as f32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * viewport_ratio;

    // Camera setup. Currently at the origin
    let camera: Point3 = Vec3::new();
    let focal_length: f32 = 1.0;
    
    // Viewport vectors. Since the pixels are numbered from the top,
    // while the camera has axis facing upwards, we are forced to mark
    // the v vector with negative direction
    let viewport_u = Vec3::from_point(viewport_width, 0.0, 0.0);
    let viewport_v = -Vec3::from_point(0.0, viewport_width, 0.0);

    let delta_u = viewport_u / image_width as f32;
    let delta_v = viewport_v / image_height as f32;

    let viewport_topleft = camera - Vec3::from_point(0.0, 0.0, focal_length)
	- viewport_u / 2.0
	- viewport_v / 2.0;

    let first_pixel = viewport_topleft + 0.5 * (delta_u + delta_v);
    
    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    for i in 0..WIDTH {
	eprint!("\rNumber of lines completed: {}", i);
	for j in 0..HEIGHT {
	    let pixel_center = first_pixel
		+ (i as f32) * delta_u
		+ (j as f32) * delta_v;

	    // Ray points 
	    let direction = pixel_center - camera;

	    let pixel = ray_colour(&Ray::construct(pixel_center, direction));
	    
	    println!("{}", pixel);
	}
    }

    eprintln!();
    eprintln!("Done.");
}
