use crate::hitrecord::HitRecord;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::figure::Figure;

/*pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}*/

pub struct World {
    pub objects: Vec<Figure>
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Figure) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_anything = Some(hit_record);
            }
        }

        hit_anything
    }
}
