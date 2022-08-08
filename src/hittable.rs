use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(p: &Point3, t: f32) -> HitRecord {
        HitRecord { p: *p, normal: Vec3::new_empty(), t: t, front_face: false }
    }

    // set the "normal" vector to be always pointing to the opposite direction of the ray
    // TODO: make `normal` private to make sure this is the only way to set `normal`
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // if the dot product is smaller than 0, then it means the ray hits the outside surface, we can keep the outward_normal
        // otherwise it means the ray hits from the inside of the surface, we should revert the direction of the outward_normal
        self.front_face = r.direction.dot(*outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -outward_normal }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}