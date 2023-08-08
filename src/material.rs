use crate::vec3::Vec3;
use crate::color::Color;
use crate::ray::Ray;
use crate::hitrecord::HitRecord;
use crate::util::reflectance;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn scatter<R: Rng>(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut R) -> Option<(Color, Ray)> {
        let target = &rec.p + &rec.normal + Vec3::rand_in_unit_sphere(rng);
        let scattered = Ray::new(rec.p.clone(), target - &rec.p);
        Some((self.albedo.clone(), scattered))
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }

    pub fn scatter<R: Rng>(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut R) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(rec.p.clone(), reflected + (self.fuzz * Vec3::rand_in_unit_sphere(rng)));
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((self.albedo.clone(), scattered))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }

    pub fn scatter<R: Rng>(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut R) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray_in.direction.unit_vector();
        let munit_direction = -(unit_direction.clone());
        let cos_theta = munit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        let ray = Ray::new(rec.p.clone(), direction);
        return Some((attenuation, ray))
    }
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter<R: Rng>(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut R) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray_in, rec, rng),
            Material::Metal(m) => m.scatter(ray_in, rec, rng),
            Material::Dielectric(d) => d.scatter(ray_in, rec, rng),
        }
    }

    pub fn lambertian(albedo: Color) -> Material {
        return Material::Lambertian(Lambertian::new(albedo));
    }

    pub fn metal(albedo: Color, fuzz: f64) -> Material {
        return Material::Metal(Metal::new(albedo, fuzz));
    }

    pub fn dielectric(ref_idx: f64) -> Material {
        return Material::Dielectric(Dielectric::new(ref_idx));
    }



}

