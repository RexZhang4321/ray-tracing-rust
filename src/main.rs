use std::fs::File;
use std::io::Write;

mod vec3;
mod color_utils;
mod ray;
mod hittable;

use ray::Ray;
use vec3::Point3;
use vec3::Vec3;
use vec3::Color;

// t^2 * b^2 + 2 * t * b * (A - C) + (A - C)^2 - r ^ 2 = 0
// where t is the time
// b is the direction of the ray
// A is the origin of the ray
// r is radius of the sphere
// when t has 1 or 2 roots, then it means the ray hits the sphere
// the formula to solve this equation is generally (-b +- sqrt(b^2 - 4ac)) / (2a)
// subtitute b with 2h
// we can get (-h +- sqrt(h^2 - ac)) / a
fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> f32 {
    let a_sub_c = r.origin - *center;
    let a = r.direction.dot(r.direction);
    let half_b = r.direction.dot(a_sub_c);
    let c = a_sub_c.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray) -> Color {
    let center = Point3 { x: 0.0, y: 0.0, z: -1.0 };
    let t_hit = hit_sphere(&center, 0.5, r);
    if t_hit > 0.0 {
        let normal = (r.at(t_hit) - center).unit_vector();
        return 0.5 * Color {x: normal.x + 1.0, y: normal.y + 1.0, z: normal.z + 1.0};
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color {x: 1.0, y: 1.0, z: 1.0} + t * Color {x: 0.5, y: 0.7, z: 1.0}
}

fn main() -> std::io::Result<()> {
    
    // image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

    // camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Point3 = Point3 {x: 0.0, y: 0.0, z: 0.0};
    let horizontal: Vec3 = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical: Vec3 = Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3 { x: 0.0, y: 0.0, z: focal_length };

    // render
    let mut buffer = String::new();

    buffer.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    for j in (0..image_height).rev() {
        // println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = Ray {origin: origin, direction: lower_left_corner + u * horizontal + v * vertical - origin};
            let pixel_color = ray_color(&r);

            color_utils::write_color(&mut buffer, &pixel_color);
        }
    }

    let mut output = File::create("image.ppm")?;
    output.write(buffer.as_bytes())?;

    Ok(())
}
