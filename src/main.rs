use color::Color;
use ray::Ray;
use vector3::{Point3, Vector3};

mod color;
mod ray;
mod vector3;
fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    assert!(image_height >= 1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let i = i as f64;
            let j = j as f64;
            let pixel_center = pixel00_loc + pixel_delta_u * i + pixel_delta_v * j;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray);
            println!("{color}");
        }
    }
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let a = (unit_direction.y + 1.0) * 0.5;

    Color(Vector3::ONE * (1.0 - a) + Vector3::new(0.5, 0.7, 1.0) * a)
}
