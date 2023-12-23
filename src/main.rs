use crate::vec3::Vec3;

mod vec3;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {
    eprintln!("Starting output print...");
    
    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    for i in 0..WIDTH {
	eprint!("\rNumber of lines completed: {}", i);
	for j in 0..HEIGHT {
	    let pixel: Vec3 = Vec3::from_point(i as f32, j as f32, 0.0);
	    
	    println!("{}", pixel / (WIDTH - 1) as f32)
	}
    }

    eprintln!();
    eprintln!("Done.");
}
