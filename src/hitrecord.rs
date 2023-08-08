use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::material::Material;
use std::rc::Rc;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a Material,
}

pub enum FaceNormal {
    Front(Vec3),
    Back(Vec3),
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool, material: &'a Material) -> Self {

        Self { p, normal, t, front_face, material }
    }

    pub fn get_face_normal(ray: &Ray, outward_normal: &Vec3) -> FaceNormal {
        if ray.direction.dot(outward_normal) < 0.0 {
            return FaceNormal::Front(outward_normal.clone());
        } else {
            return FaceNormal::Back(-outward_normal);
        }
    }
}
