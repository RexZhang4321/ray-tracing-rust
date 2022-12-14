use std::rc::Rc;

use crate::hittable::*;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {

    // TODO: this is probably an incorrect implementation because only the last hit object info is preserved
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None::<HitRecord>;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            match object.hit(r, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t;
                    temp_rec = Some(rec);
                },
                None => (),
            }
        }

        temp_rec
    }
}