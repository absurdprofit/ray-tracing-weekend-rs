pub mod dielectric;
pub mod lambertian;
pub mod metal;
pub mod vanta_black;
use crate::{
    colour::Colour,
    hittable::HitRecord,
    material::{
        dielectric::Dielectric, lambertian::Lambertian, metal::Metal, vanta_black::VantaBlack,
    },
    ray::Ray,
};

pub trait Material {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}

pub enum MaterialKind {
    VantaBlack(VantaBlack),
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
}

impl Material for MaterialKind {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            MaterialKind::VantaBlack(m) => m.scatter(r_in, rec, attenuation, scattered),
            MaterialKind::Metal(m) => m.scatter(r_in, rec, attenuation, scattered),
            MaterialKind::Lambertian(m) => m.scatter(r_in, rec, attenuation, scattered),
            MaterialKind::Dielectric(m) => m.scatter(r_in, rec, attenuation, scattered),
        }
    }
}
