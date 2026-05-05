use crate::{colour::Colour, hittable::HitRecord, material::Material, ray::Ray};

pub struct VantaBlack;

impl Material for VantaBlack {
    fn scatter(
        &self,
        _r_in: Ray,
        _rec: &HitRecord,
        _attenuation: &mut Colour,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}
