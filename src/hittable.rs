use crate::vec3::*;

pub struct Hittable {
  pub point: Point3,
  pub normal: Vec3,
  pub material: Material,
  pub t: f64,
  pub front_face: bool
}

impl Hittable {
  pub fn set_face_normal(self: &Self, ray: &Ray, outward_normal: Vec3) {
    // outward_normal is assumed to have unit length
    self.front_face = dot(ray.direction(), &outward_normal) < 0.0;
    self.normal = match self.front_face {
      true => outward_normal,
      false => -outward_normal
    };
  }
}