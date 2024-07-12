use color::Color;
use vector3::Vector3;

mod color;
mod ray;
mod vector3;

fn main() {
    let color = Color(Vector3 {
        x: 0.5,
        y: 0.75,
        z: 1.0,
    });
    eprintln!("{color}");

    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let col = Color(Vector3 {
                x: i as f64 / (image_width - 1) as f64,
                y: j as f64 / (image_height - 1) as f64,
                z: 0.0,
            });

            println!("{col}");
        }
    }
}
