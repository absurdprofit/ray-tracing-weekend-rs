pub mod hit_record;
pub mod sphere;
use enum_dispatch::enum_dispatch;

use crate::{
    hittable::{hit_record::HitRecord, sphere::Sphere},
    interval::Interval,
    ray::Ray,
};

#[enum_dispatch]
pub trait Hittable {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord<'a>) -> bool;
}

#[enum_dispatch(Hittable)]
pub enum HittableKind {
    Sphere,
}
