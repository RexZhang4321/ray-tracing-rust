use std::rc::Rc;

use crate::material::Material;
use crate::vec3::*;
use crate::hittable::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Rc<dyn Material>
}

impl Hittable for Sphere {

    // t^2 * b^2 + 2 * t * b * (A - C) + (A - C)^2 - r ^ 2 = 0
    // where t is the time
    // b is the direction of the ray
    // A is the origin of the ray
    // r is radius of the sphere
    // when t has 1 or 2 roots, then it means the ray hits the sphere
    // the formula to solve this equation is generally (-b +- sqrt(b^2 - 4ac)) / (2a)
    // subtitute b with 2h
    // we can get (-h +- sqrt(h^2 - ac)) / a
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let a_sub_c = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let half_b = r.direction.dot(a_sub_c);
        let c = a_sub_c.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        // find the nearest root that lies in the acceptable range
        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut rec = HitRecord::new(&r.at(root), root, Rc::clone(&self.material));
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}