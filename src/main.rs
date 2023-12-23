const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {
    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    for i in 0..WIDTH {
	for j in 0..HEIGHT {
	    let r: u32 = (256.00 * (i as f32) / (WIDTH - 1) as f32) as u32;
	    let g: u32 = (256.00 * (j as f32) / (WIDTH - 1) as f32) as u32;
	    let b: u32 = 0;

	    println!("{} {} {}", r, g, b);
	}
    }
}
