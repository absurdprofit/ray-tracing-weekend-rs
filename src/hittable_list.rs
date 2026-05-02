use crate::{hittable::{HitRecord, Hittable}, interval::Interval, material::{MaterialKind, vanta_black::VantaBlack}, ray::Ray, vec3::{Point3, Vec3}};

pub struct HittableList<T: Hittable> {
  pub objects: Vec<T>,
}

const DEFAULT_MATERIAL: MaterialKind = MaterialKind::VantaBlack(VantaBlack);
impl<T: Hittable> HittableList<T> {
  pub fn new() -> Self {
    Self {
      objects: vec![]
    }
  }

  pub fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'a>> {
    let mut hit_anything = false;
    let mut closest_so_far = ray_t.max;
    let mut rec = HitRecord {
      point: Point3::new(),
      normal: Vec3::new(),
      material: &DEFAULT_MATERIAL,
      t: 0.0,
      front_face: false
    };

    for object in &self.objects {
      if object.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut rec) {
        closest_so_far = rec.t;
        hit_anything = true;
      }
    }

    if hit_anything {
      return Some(rec)
    } else {
      return None
    }
  }
}