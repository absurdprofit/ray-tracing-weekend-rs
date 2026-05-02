mod hittable;
mod vec3;
mod common;
mod ray;
mod hittable_list;
mod interval;
mod material;
mod colour;
mod sphere;
mod camera;
use crate::{camera::Camera, colour::Colour, hittable_list::HittableList, material::{MaterialKind, lambertian::Lambertian, metal::Metal}, sphere::Sphere, vec3::{Point3, Vec3}};
use std::fs::File;
use std::io::{BufWriter, Result};

fn main() -> Result<()> {
    let file = File::create("image.ppm")?;
    let writer = BufWriter::new(file);
    let mut camera = Camera::new(writer);
    let mut world = HittableList::new();

    let material_ground = MaterialKind::Lambertian(
        Lambertian::new(Colour::from_tuple((0.8, 0.8, 0.0)))
    );
    let material_centre = MaterialKind::Lambertian(
        Lambertian::new(Colour::from_tuple((0.1, 0.2, 0.5)))
    );
    // let material_left = make_shared<dielectric>(1.50);
    // let material_bubble = make_shared<dielectric>(1.00 / 1.50);
    let material_right = MaterialKind::Metal(
        Metal::new(Colour::from_tuple((0.8, 0.6, 0.2)), 1.0)
    );
    // auto material_vanta_black = make_shared<vanta_black>();

    world.objects.push(
        Sphere::new(Point3::from_tuple((0.0, -100.5, -1.0)), 100.0, material_ground)
    );
    world.objects.push(
        Sphere::new(Point3::from_tuple((0.0, 0.0, -1.2)), 0.5, material_centre)
    );
    // world.add(make_shared<sphere>(point3(-1.0, 0.0, -1.0), 0.5, material_left));
    // world.add(make_shared<sphere>(point3(-1.0, 0.0, -1.0), 0.4, material_bubble));
    world.objects.push(
        Sphere::new(Point3::from_tuple((1.0, 0.0, -1.0)), 0.5, material_right)
    );
    // world.add(make_shared<sphere>(point3(2.0, 0.5, -2.0), 0.5, material_vanta_black));

    camera.set_image_width(400);
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.v_fov_deg = 20.0;
    camera.look_from = Point3::from_tuple((-2.0, 2.0, 1.0));
    camera.look_at = Point3::from_tuple((0.0, 0.0, -1.0));
    camera.v_up = Vec3::from_tuple((0.0, 1.0, 0.0));

    camera.defocus_angle_deg = 10.0;
    camera.focus_distance = 3.4;

    return camera.render(&world);
}
