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
    let aspect_ratio: f32 = 3.0 / 2.0;
    let image_width: i32 = 1200;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;
    let max_depth: i32 = 50;

    // world
    let world = random_scene();

    // camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_to = Point3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        look_from,
        look_to,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0
    );

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

fn random_scene() -> HittableList {
    let mut world = HittableList { objects: Vec::new() };

    let ground_material = Rc::new(Lambertian {albedo: Color::new(0.5, 0.5, 0.5)});
    world.add(Rc::new(Sphere { center: Point3 {x: 0.0, y: -1000.0, z: 0.0}, radius: 1000.0, material: ground_material }));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Point3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian {albedo: albedo});
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz: f32 = rng.gen_range(0.0..0.5);
                    sphere_material = Rc::new(Metal {albedo, fuzz});
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric {ir: 1.5});
                }
                world.add(Rc::new(Sphere {center: center, radius: 0.2, material: sphere_material}));
            }
        }
    }

    let material_1 = Rc::new(Dielectric {ir: 1.5});
    world.add(Rc::new(Sphere {center: Point3::new(0.0, 1.0, 0.0), radius: 1.0, material: material_1}));

    
    let material_2 = Rc::new(Lambertian {albedo: Color::new(0.4, 0.2, 0.1)});
    world.add(Rc::new(Sphere {center: Point3::new(-4.0, 1.0, 0.0), radius: 1.0, material: material_2}));

    
    let material_3 = Rc::new(Metal {albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0});
    world.add(Rc::new(Sphere {center: Point3::new(4.0, 1.0, 0.0), radius: 1.0, material: material_3}));
    world
}