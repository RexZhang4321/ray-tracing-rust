use crate::vec3::*;
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}

impl Camera {
    // vup -> view up vector, or the direction of the "up view", think of rotating the head around the nose axle
    // (0, 1, 0) means look from horizontal, up from the gravity
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);
    
        let origin: Point3 = look_from;
        let horizontal: Vec3 = focus_dist * viewport_width * u;
        let vertical: Vec3 = focus_dist * viewport_height * v;
        let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        // larget the lens_radius is, the more blur out of focus
        let lens_radius = aperture / 2.0;
        Camera { origin: origin, lower_left_corner: lower_left_corner, horizontal: horizontal,
            vertical: vertical, u: u, v: v, w: w, lens_radius: lens_radius }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {origin: self.origin + offset,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset}
    }
}