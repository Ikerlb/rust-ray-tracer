use crate::hitrecord::{HitRecord, FaceNormal};
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::material::Material;

use std::rc::Rc;

pub enum Figure {
    Sphere(Sphere)
}

impl Figure {
    pub fn sphere(center: Vec3, radius: f64, mat: Material) -> Self {
        Self::Sphere(Sphere::new(center, radius, mat))
    }


    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Figure::Sphere(sphere) => sphere.hit(ray, t_min, t_max)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self { center, radius, material }
    }
}

impl Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (&point - &self.center) / self.radius;
        match HitRecord::get_face_normal(ray, &outward_normal) {
            FaceNormal::Front(normal) => Some(HitRecord::new(point, normal, root, true, &self.material)),
            FaceNormal::Back(normal) => Some(HitRecord::new(point, normal, root, false, &self.material))
        }
    }

}
