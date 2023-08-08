mod vec;
mod vec3;
mod color;
mod ray;
mod world;
mod hitrecord;
mod camera;
mod util;
mod material;
mod figure;

use std::fs::File;
use std::io::{BufWriter, Write};

use clap::Parser;
use vec3::Vec3;
use color::Color;
use ray::Ray;
use world::World;
use figure::{Figure, Sphere};
use camera::Camera;
use material::{Material, Lambertian, Metal, Dielectric};

use rayon::prelude::*;
use std::rc::Rc;
use rand::{Rng, self};

/// Simple rust ray tracer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Recursion depth limit for rays
    #[arg(short, long)]
    max_depth: Option<u8>,

    /// Width of the output image
    #[arg(short, long)]
    width: Option<usize>,

    /// Height of the output image
    #[arg(short, long)]
    height: Option<usize>,

    /// Number of samples per pixel
    #[arg(short, long)]
    samples: Option<usize>,

    /// Output file name
    #[arg(short, long)]
    file: std::path::PathBuf,
}

//TODO: Change rand unit vector to random in hemisphere!!!
//TODO: Fuzz is the min between 1 and the fuzz
fn ray_color<R: Rng>(r: &Ray, world: &World, rng: &mut R, depth: u8) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec, rng) {
            return attenuation * ray_color(&scattered, world, rng, depth - 1);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5f64 * (unit_direction.y() + 1.0f64);
    let white = Color::new_color(1.0, 1.0, 1.0);
    let col = Color::new_color(0.5, 0.7, 1.0);
    return ((1.0f64 - t) * &white) + (t * &col);
}

fn create_final_world<R: Rng>(rng: &mut R, aspect_ratio: f64) -> (World, Camera) {
    let mut world = World::new();
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),  // lookfrom
        Vec3::new(0.0, 0.0, 0.0), // lookat
        Vec3::new(0.0, 1.0, 0.0),  // up
        20.0,                      // fov
        aspect_ratio);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..=1.0);
            let centerx = (a as f64) + (0.9 * rng.gen_range(0.0..=1.0));
            let centery = 0.2;
            let centerz = (b as f64) + (0.9 * rng.gen_range(0.0..=1.0));
            let center = Vec3::new(centerx, centery, centerz);
            if (&center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::rand(rng) * Color::rand(rng);
                    let sphere_mat = Material::lambertian(albedo);
                    let sphere = Sphere::new(center, 0.2, sphere_mat);
                    world.add(Figure::Sphere(sphere));
                } else if choose_mat < 0.95 {
                    let albedo = Color::rand_range(rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let sphere_mat = Material::metal(albedo, fuzz);
                    let sphere = Sphere::new(center, 0.2, sphere_mat);
                    world.add(Figure::Sphere(sphere));
                } else {
                    let sphere_mat = Material::dielectric(1.5);
                    let sphere = Sphere::new(center, 0.2, sphere_mat);
                    world.add(Figure::Sphere(sphere));
                }
            }
        }
    }

    let alb1 = Color::new(0.4, 0.2, 0.1);
    let mat1 = Material::lambertian(alb1); 
    let sphere1 = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat1);
    world.add(Figure::Sphere(sphere1));
    
    let alb2 = Color::new(0.7, 0.6, 0.5);
    let mat2 = Material::metal(alb2, 0.0); 
    let sphere2 = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat2);
    world.add(Figure::Sphere(sphere2));

    let mat3 = Material::dielectric(1.5);
    let sphere3 = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat3);
    world.add(Figure::Sphere(sphere3));

    let ground_alb = Color::new(0.5, 0.5, 0.5);
    let ground_mat = Material::lambertian(ground_alb);
    let sphere = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);
    world.add(Figure::Sphere(sphere));
    (world, camera)
}

fn create_world_with_three_spheres() -> (World, Camera) {

    let mat_ground = Material::lambertian(Color::new_color(0.8, 0.8, 0.0));
    let mat_center = Material::lambertian(Color::new_color(0.1, 0.2, 0.5));
    let mat_left_1 = Material::dielectric(1.5);
    let mat_left_2 = Material::dielectric(1.5);
    let mat_right = Material::metal(Color::new_color(0.8, 0.6, 0.2), 0.0);


    let mut world = World::new();
    let sphere_ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere2_1 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left_1);
    let sphere2_2 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, mat_left_2);
    let sphere3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),  // lookfrom
        Vec3::new(0.0, 0.0, -1.0), // lookat
        Vec3::new(0.0, 1.0, 0.0),  // up
        20.0,                      // fov
        16.0 / 9.0);

    world.add(Figure::Sphere(sphere_ground));
    world.add(Figure::Sphere(sphere1));
    world.add(Figure::Sphere(sphere2_1));
    world.add(Figure::Sphere(sphere2_2));
    world.add(Figure::Sphere(sphere3));
    (world, camera)
}

fn create_two_spheres_world() -> (World, Camera) {

    let mat_left = Material::lambertian(Color::new_color(0.0, 0.0, 1.0));
    let mat_right = Material::lambertian(Color::new_color(1.0, 0.0, 0.0));

    let r = (std::f64::consts::PI / 4.0).cos();
    let mut world = World::new();
    let sphere_left = Sphere::new(Vec3::new(-r, 0.0, -1.0), r, mat_left);
    let sphere_right = Sphere::new(Vec3::new(r, 0.0, -1.0), r, mat_right);

    world.add(Figure::Sphere(sphere_left));
    world.add(Figure::Sphere(sphere_right));

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0), // lookfrom
        Vec3::new(0.0, 0.0, -1.0), // lookat
        Vec3::new(0.0, 1.0, 0.0),  // up
        90.0,                      // fov
        16.0 / 9.0);

    (world, camera)
}

/*
 * I like the way the way we can use
 * both owned vec3 and borrowed vec3
 * with operators like + and -
 * and that we did so without 
 * implementing copy
 */
fn main() {
    let args = Args::parse();

    //let image_width = 1200;
    let image_width = args
        .width
        .unwrap_or(200);
    let image_height = args
        .height
        .unwrap_or(200);
    let max_depth = args
        .max_depth
        .unwrap_or(10);
    let samples_per_pixel = args
        .samples
        .unwrap_or(50);

    let file_name = args.file;

    let aspect_ratio = image_width as f64 / image_height as f64;


    let mut first_rng = rand::thread_rng();
    let (world, camera) = create_final_world(&mut first_rng, aspect_ratio);

    let file = File::create(file_name).unwrap();
    let mut writer = BufWriter::new(&file);

    writeln!(&mut writer, "P3\n{} {}\n255", image_width, image_height);

    let res = (0..image_height)
        .into_par_iter()
        .map(|row| {
            let percent = (image_height as f64 - row as f64) / (image_height as f64) * 100.0;
            let mut rng = rand::thread_rng();
            return (0..image_width).map(|col| {
                let mut pixel_color = Color::new_color(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {

                    let ru = rng.gen_range(0.0..=1.0);
                    let rv = rng.gen_range(0.0..=1.0);

                    let u = (col as f64 + ru) / (image_width-1) as f64;
                    let v = (row as f64 + rv)/ (image_height-1) as f64;

                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, &world, &mut rng, max_depth);
                }
                return format!("{}\n", pixel_color.ppm_color(samples_per_pixel as f64));
            }).collect::<String>()
        })
        .rev()
        .collect::<String>();

    writeln!(&mut writer, "{}", res);
}
