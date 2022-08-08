use std::fs::File;
use std::io::Write;
use std::rc::Rc;

mod vec3;
mod color_utils;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;

use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use vec3::Point3;
use vec3::Vec3;
use vec3::Color;
use sphere::Sphere;

fn ray_color(r: &Ray, hittable: &impl Hittable) -> Color {
    match hittable.hit(r, 0.0, f32::MAX) {
        Some(rec) => {
            return 0.5 * Color {x: rec.normal.x + 1.0, y: rec.normal.y + 1.0, z: rec.normal.z + 1.0};
        },
        None => (),
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

    // world
    let mut world = HittableList { objects: Vec::new() };
    world.add(Rc::new(Sphere { center: Point3 {x: 0.0, y: 0.0, z: -1.0}, radius: 0.5 }));
    world.add(Rc::new(Sphere { center: Point3 {x: 0.0, y: -100.5, z: -1.0}, radius: 100.0 }));

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
            let pixel_color = ray_color(&r, &world);

            color_utils::write_color(&mut buffer, &pixel_color);
        }
    }

    let mut output = File::create("image.ppm")?;
    output.write(buffer.as_bytes())?;

    Ok(())
}
