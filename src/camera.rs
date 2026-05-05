use crate::{
    colour::{Colour, write_colour},
    common::{INFINITY, degrees_to_radians, random_double},
    hittable_list::HittableList,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3, cross, random_in_unit_disk, unit_vector},
};
use std::io::{BufWriter, Result, Write};

pub enum Resolution {
    SD,
    HD,
    FHD,
    QHD,
    UHD,
}

pub struct Camera<W: Write> {
    writer: BufWriter<W>,
    image_width: u16,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u8,
    pub max_depth: i16,
    pub v_fov_deg: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub v_up: Vec3,
    pub defocus_angle_deg: f64,
    pub focus_distance: f64,
}

impl<W: Write> Camera<W> {
    pub fn new(writer: BufWriter<W>) -> Self {
        let look_from = Point3::from_tuple((0.0, 0.0, 0.0));
        let look_at = Point3::from_tuple((0.0, 0.0, -1.0));
        let v_up = Vec3::from_tuple((0.0, 1.0, 0.0));
        let defocus_angle_deg = 0.0;
        let focus_distance = 10.0;
        Self {
            writer,
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,
            v_fov_deg: 90.0,
            look_from,
            look_at,
            v_up,
            defocus_angle_deg,
            focus_distance,
        }
    }

    pub fn render(&mut self, world: &HittableList) -> Result<()> {
        let image_height = self.image_height();
        self.writer.write_all(b"P3\n")?;
        self.writer
            .write_all(self.image_width.to_string().as_bytes())?;
        self.writer.write_all(b" ")?;
        self.writer.write_all(image_height.to_string().as_bytes())?;
        self.writer.write_all(b" ")?;
        self.writer.write_all(b"\n255\n")?;
        for j in 0..image_height {
            let percentage = (j as f64 / (image_height - 1) as f64) * 100.0;
            print!("\rRendering: {:.2}%", percentage);
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::new();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_colour += self.ray_colour(r, self.max_depth, world);
                }
                let pixel_colour = self.pixel_samples_scale() * pixel_colour;

                write_colour(&mut self.writer, pixel_colour)?;
            }
        }

        self.writer.flush()?;

        Ok(())
    }

    pub fn set_resolution(&mut self, resolution: Resolution) {
        let image_width = match resolution {
            Resolution::SD => 854,
            Resolution::HD => 1280,
            Resolution::FHD => 1920,
            Resolution::QHD => 2560,
            Resolution::UHD => 3840,
        };
        self.set_aspect_ratio(16.0 / 9.0);
        self.set_image_width(image_width);
    }

    pub fn set_image_width(&mut self, image_width: u16) {
        let aspect_ratio = self.aspect_ratio as u16;
        self.image_width = image_width;
        if self.image_width < aspect_ratio {
            self.image_width = aspect_ratio;
        }
    }
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = aspect_ratio;
        self.set_image_width(self.image_width); // trigger image width validation
    }

    fn w(&self) -> Vec3 {
        unit_vector(self.look_from.clone() - self.look_at.clone())
    }
    fn u(&self) -> Vec3 {
        unit_vector(cross(&self.v_up, &self.w()))
    }
    fn v(&self) -> Vec3 {
        cross(&self.w(), &self.u())
    }
    fn defocus_disk_u(&self) -> Vec3 {
        self.u().clone() * self.defocus_radius()
    }
    fn defocus_disk_v(&self) -> Vec3 {
        self.v().clone() * self.defocus_radius()
    }
    fn defocus_radius(&self) -> f64 {
        self.focus_distance * degrees_to_radians(self.defocus_angle_deg / 2.0).tan()
    }
    fn image_height(&self) -> u16 {
        let aspect_ratio = self.aspect_ratio;

        (self.image_width as f64 / aspect_ratio) as u16
    }
    fn pixel_samples_scale(&self) -> f64 {
        1.0 / self.samples_per_pixel as f64
    }
    fn centre(&self) -> &Point3 {
        &self.look_from
    }
    fn pixel00_loc(&self) -> Point3 {
        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.centre().clone()
            - (self.focus_distance * self.w().clone())
            - self.viewport_u() / 2.0
            - self.viewport_v() / 2.0;
        viewport_upper_left + 0.5 * (self.pixel_delta_u() + self.pixel_delta_v())
    }
    fn viewport_height(&self) -> f64 {
        let theta = degrees_to_radians(self.v_fov_deg);
        let h = (theta / 2.0).tan();
        2.0 * h * self.focus_distance
    }
    fn viewport_width(&self) -> f64 {
        self.viewport_height() * self.aspect_ratio
    }
    fn viewport_u(&self) -> Vec3 {
        self.viewport_width() * self.u().clone() // Vector across viewport horizontal edge
    }
    fn viewport_v(&self) -> Vec3 {
        self.viewport_height() * -self.v().clone() // Vector down viewport vertical edge
    }
    fn pixel_delta_u(&self) -> Vec3 {
        self.viewport_u() / self.image_width as f64
    }
    fn pixel_delta_v(&self) -> Vec3 {
        self.viewport_v() / self.image_height() as f64
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::from_tuple((random_double() - 0.5, random_double() - 0.5, 0.0))
    }

    fn get_ray(&self, i: u16, j: u16) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly
        // sampled point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc()
            + ((i as f64 + offset.x()) * self.pixel_delta_u())
            + ((j as f64 + offset.y()) * self.pixel_delta_v());

        let ray_origin = if self.defocus_angle_deg <= 0.0 {
            self.centre().clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin.clone();

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.centre().clone()
            + (p.x() * self.defocus_disk_u().clone() + (p.y() * self.defocus_disk_v().clone()))
    }

    fn ray_colour(&self, r: Ray, depth: i16, world: &HittableList) -> Colour {
        if depth <= 0 {
            return Colour::new();
        }

        if let Some(rec) = world.hit(&r, &Interval::new(0.001, INFINITY)) {
            let mut scattered = Ray::new(Vec3::new(), Vec3::new());
            let mut attenuation = Colour::new();
            if rec
                .material
                .scatter(r.clone(), &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * self.ray_colour(scattered, depth - 1, world);
            }
            return Colour::new();
        }
        let unit_direction = unit_vector(r.direction().clone());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Colour::from_tuple((1.0, 1.0, 1.0)) + a * Colour::from_tuple((0.5, 0.7, 1.0))
    }
}
