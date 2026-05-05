pub mod dielectric;
pub mod lambertian;
pub mod metal;
pub mod vanta_black;
use enum_dispatch::enum_dispatch;

use crate::{
    colour::Colour,
    hittable::hit_record::HitRecord,
    material::{
        dielectric::Dielectric, lambertian::Lambertian, metal::Metal, vanta_black::VantaBlack,
    },
    ray::Ray,
};

#[enum_dispatch]
pub trait Material {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}

#[enum_dispatch(Material)]
pub enum MaterialKind {
    VantaBlack,
    Metal,
    Lambertian,
    Dielectric,
}
