use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::util::degrees_to_radians;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
        //let image_height = (image_width as f64 / aspect_ratio) as i64;

        let focal_length = 1.0;
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = aspect * viewport_height;

        let w = (&lookfrom - &lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = &origin - (&horizontal / 2.0) - (&vertical / 2.0) - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let origin = self.origin.clone();
        let direction = &self.lower_left_corner + (&self.horizontal * u) + (&self.vertical * v) - &self.origin;
        Ray::new(origin, direction)
    }

}
