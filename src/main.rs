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
mod material;

use hittable::Hittable;
use hittable_list::HittableList;
use rand::Rng;
use ray::Ray;
use vec3::Point3;
use vec3::Vec3;
use vec3::Color;
use sphere::Sphere;
use camera::Camera;
use material::*;

fn ray_color(r: &Ray, hittable: &impl Hittable, depth: i32) -> Color {

    // if we've exceeded the ray bouncing limit, we will not gather more light
    if depth <= 0 {
        return Color::black();
    }

    // some of the reflected rays hit the object they are reflecting off of not at exactly t = 0,
    // but something extremely close to 0 (shadow acne problem)
    match hittable.hit(r, 0.001, f32::MAX) {
        Some(rec) => {
            match rec.material.scatter(r, &rec) {
                Some((attenuation, scattered)) => {
                    return attenuation * ray_color(&scattered, hittable, depth - 1);
                },
                None => return Color::white()
            }
        },
        None => (),
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::black() + t * Color {x: 0.5, y: 0.7, z: 1.0}
}

fn main() -> std::io::Result<()> {
    
    // image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;
    let max_depth: i32 = 50;

    // world
    let mut world = HittableList { objects: Vec::new() };

    let material_ground = Rc::new(Lambertian {albedo: Color::new(0.8, 0.8, 0.0)});
    let material_center = Rc::new(Lambertian {albedo: Color::new(0.1, 0.2, 0.5)});
    // let material_left = Rc::new(Metal {albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0.3});
    //let material_center = Rc::new(Dielectric {ir: 1.5});
    let material_left = Rc::new(Dielectric {ir: 1.5});
    let material_right = Rc::new(Metal {albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0});

    world.add(Rc::new(Sphere { center: Point3 {x: 0.0, y: -100.5, z: -1.0}, radius: 100.0, material: material_ground }));
    world.add(Rc::new(Sphere { center: Point3 {x: 0.0, y: 0.0, z: -1.0}, radius: 0.5, material: material_center}));
    world.add(Rc::new(Sphere { center: Point3 {x: -1.0, y: 0.0, z: -1.0}, radius: 0.5, material: Rc::clone(&material_left) as Rc<dyn Material>}));
    world.add(Rc::new(Sphere { center: Point3 {x: -1.0, y: 0.0, z: -1.0}, radius: -0.4, material: Rc::clone(&material_left) as Rc<dyn Material>}));
    world.add(Rc::new(Sphere { center: Point3 {x: 1.0, y: 0.0, z: -1.0}, radius: 0.5, material: material_right}));


    // camera
    let camera = Camera::new();

    // render
    let mut buffer = String::new();
    let mut rng = rand::thread_rng();

    buffer.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new_empty();
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            color_utils::write_color(&mut buffer, &pixel_color, samples_per_pixel);
        }
    }

    let mut output = File::create("image.ppm")?;
    output.write(buffer.as_bytes())?;

    Ok(())
}
