use crate::vec3::*;
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    // vup -> view up vector, or the direction of the "up view", think of rotating the head around the nose axle
    // (0, 1, 0) means look from horizontal, up from the gravity
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);
    
        let origin: Point3 = look_from;
        let horizontal: Vec3 = viewport_width * u;
        let vertical: Vec3 = viewport_height * v;
        let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Camera { origin: origin, lower_left_corner: lower_left_corner, horizontal: horizontal, vertical: vertical }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {origin: self.origin, direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin}
    }
}