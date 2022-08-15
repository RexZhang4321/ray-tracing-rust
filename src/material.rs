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


pub struct Dielectric {
    // index of refaction
    pub ir: f32
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::black();
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };
        
        let unit_direction = r_in.direction.unit_vector();

        let cos_theta = (-unit_direction.dot(rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;
        
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random::<f32>() {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        Some((attenuation, Ray {origin: rec.p, direction: direction}))
    }
}

impl Dielectric {

    // Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0_square = r0 * r0;
        r0_square + (1.0 - r0_square) * (1.0 - cosine).powi(5)
    }
}