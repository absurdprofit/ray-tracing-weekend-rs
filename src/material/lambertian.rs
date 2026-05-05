use crate::{
    colour::Colour, hittable::HitRecord, material::Material, ray::Ray, vec3::random_unit_vector,
};

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal.clone() + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        *scattered = Ray::new(rec.point.clone(), scatter_direction);
        *attenuation = self.albedo.clone();

        true
    }
}
