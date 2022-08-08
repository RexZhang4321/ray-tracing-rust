use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32
}

trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> bool;
}