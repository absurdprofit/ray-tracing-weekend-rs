use crate::{hittable::{HitRecord, Hittable}, interval::Interval, material::MaterialKind, ray::Ray, vec3::{Point3, dot}};

pub struct Sphere {
  centre: Point3,
  radius: f64,
  material: MaterialKind
}

impl Sphere {
  pub fn new(centre: Point3, radius: f64, material: MaterialKind) -> Self {
    Self {
      centre,
      radius,
      material
    }
  }
}

impl Hittable for Sphere {
  fn hit<'a>(
    &'a self,
    r: &Ray,
    ray_t: &Interval,
    rec: &mut HitRecord<'a>
  ) -> bool {
    let oc = self.centre.clone() - r.origin().clone();
    let a: f64 = r.direction().length_squared();
    let h = dot(&r.direction(), &oc);
    let c = oc.length_squared() - self.radius * self.radius;

    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
      return false;
    }

    let sqrtd = discriminant.sqrt();

    let mut root = (h - sqrtd) / a;
    if !ray_t.surrounds(root) {
      root = (h + sqrtd) / a;
      if !ray_t.surrounds(root) {
        return false;
      }
    }

    rec.t = root;
    rec.point = r.at(rec.t);
    let outward_normal = (rec.point.clone() - self.centre.clone()) / self.radius;
    rec.set_face_normal(r.clone(), outward_normal);
    rec.material = &self.material;

    true
  }
}