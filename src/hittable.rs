use crate::{interval::Interval, material::MaterialKind, ray::Ray, vec3::*};

pub struct HitRecord<'a> {
  pub point: Point3,
  pub normal: Vec3,
  pub material: &'a MaterialKind,
  pub t: f64,
  pub front_face: bool
}

impl<'a> HitRecord<'a> {
  pub fn set_face_normal(self: &mut Self, ray: Ray, outward_normal: Vec3) {
    // outward_normal is assumed to have unit length
    self.front_face = dot(&ray.direction(), &outward_normal) < 0.0;
    self.normal = match self.front_face {
      true => outward_normal,
      false => -outward_normal
    };
  }
}

pub trait Hittable {
  fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord<'a>) -> bool;
}