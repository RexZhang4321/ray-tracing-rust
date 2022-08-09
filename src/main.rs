use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use rand;

mod vec3;
mod color_utils;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;

use hittable::Hittable;
use hittable_list::HittableList;
use rand::Rng;
use ray::Ray;
use vec3::Point3;
use vec3::Vec3;
use vec3::Color;
use sphere::Sphere;
use camera::Camera;

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
    let samples_per_pixel: i32 = 100;

    // world
    let mut world = HittableList { objects: Vec::new() };
    world.add(Rc::new(Sphere { center: Point3 {x: 0.0, y: 0.0, z: -1.0}, radius: 0.5 }));
    world.add(Rc::new(Sphere { center: Point3 {x: 0.0, y: -100.5, z: -1.0}, radius: 100.0 }));

    // camera
    let camera = Camera::new();

    // render
    let mut buffer = String::new();
    let mut rng = rand::thread_rng();

    buffer.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    for j in (0..image_height).rev() {
        // println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new_empty();
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            color_utils::write_color(&mut buffer, &pixel_color, samples_per_pixel);
        }
    }

    let mut output = File::create("image.ppm")?;
    output.write(buffer.as_bytes())?;

    Ok(())
}
