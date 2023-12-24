use vec3::{Vec3, Point3};
use ray::Ray;
use sphere::Sphere;

mod vec3;
mod ray;
mod sphere;

type Colour = Vec3;

const S: Sphere = Sphere {
    centre: Vec3::from_point(0.0, 0.0, -1.0),
    radius: 0.3
};

#[allow(dead_code)]
fn ray_colour (r: &Ray) -> Colour {
    match S.hit(r) {
	Some(t) => {
	    let N = (r.at(t) - Vec3::from_point(0.0, 0.0, -1.0)).unit();

	    0.5 * (N + Vec3::from_point(1.0, 1.0, 1.0))
	},
	None => {
	    let dir = r.direction.unit();
	    let a = 0.5 * (dir.y + 1.0);

	    (1.0 - a) * Vec3::from_point(1.0, 1.0, 1.0) + a * Vec3::from_point(0.5, 0.7, 1.0)
	}
    }
}

fn main() {
    eprintln!("Starting output print...");

    // Camera setup. Currently at the origin
    let camera: Point3 = Vec3::new();
    let focal_length: f32 = 1.0;

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 1000;
    
    // TODO: We currently ignore case when height is < 1
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    let viewport_ratio: f32 = image_width as f32 / image_height as f32;

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * viewport_ratio;
    
    // Viewport vectors. Since the pixels are numbered from the top,
    // while the camera has axis facing upwards, we are forced to mark
    // the v vector with negative direction
    let viewport_u: Vec3 = Vec3::from_point(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = -Vec3::from_point(0.0, viewport_height, 0.0);

    let delta_u = viewport_u / (image_width as f32);
    let delta_v = viewport_v / (image_height as f32);

    let viewport_topleft = camera - (Vec3::from_point(0.0, 0.0, focal_length)
	+ viewport_u / 2.0
	+ viewport_v / 2.0);
    
    let first_pixel = viewport_topleft + 0.5 * (delta_u + delta_v);

    eprintln!("Image properties:");
    eprintln!("\tHeight: {}", image_height);
    eprintln!("\tWidth: {}", image_width);
    eprintln!("\tAspect Ratio: {}", aspect_ratio);
    eprintln!("\tViewport Height: {}", viewport_height);
    eprintln!("\tViewport Width: {}", viewport_width);
    eprintln!("\tViewport Aspect Ratio: {}", viewport_ratio);

    eprintln!("Debug Info:");
    eprintln!("\tPixel width: {}", delta_u);
    eprintln!("\tPixel height: {}", delta_v);
    eprintln!("\tViewport width vector: {}", viewport_u);
    eprintln!("\tViewport height vector: {}", viewport_v);
    eprintln!("\tCamera Position: {}", camera);
    eprintln!("\tFocal Length: {}", focal_length);
    
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height{
	eprint!("\rNumber of lines remaining: {}", image_height - j);

	for i in 0..image_width {
	    let pixel_center = first_pixel
		+ (i as f32) * delta_u
		+ (j as f32) * delta_v;

	    let pixel = ray_colour(&Ray::construct(camera, pixel_center - camera));
	    
	    println!("{}", pixel);
	}
    }

    eprintln!();
    eprintln!("Done.");
}
