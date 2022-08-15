use crate::{hittable::HitRecord, ray::Ray, vec3::{Color, Vec3}};

pub trait Material {
    // return attenuation color and scattered ray
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // when the random_unit_vector is exactly the opposite direct of the hit normal
        // we will just use the hit normal as the scatter ray direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {origin: rec.p, direction: scatter_direction};
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.direction.unit_vector(), &rec.normal);
        let scattered = Ray {origin: rec.p, direction: reflected + self.fuzz.clamp(0.0, 1.0) * Vec3::random_in_unit_sphere()};
        let attenuation = self.albedo;

        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
