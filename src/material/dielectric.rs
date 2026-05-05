use crate::{
    colour::Colour,
    common::random_double,
    material::Material,
    ray::Ray,
    vec3::{dot, reflect, refract, unit_vector},
};

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut crate::colour::Colour,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        *attenuation = Colour::from_tuple((1.0, 1.0, 1.0));
        let ri = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = unit_vector(r_in.direction().clone());
        let cos_theta = dot(&(-unit_direction.clone()), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            reflect(unit_direction, rec.normal.clone())
        } else {
            refract(unit_direction, rec.normal.clone(), ri)
        };

        *scattered = Ray::new(rec.point.clone(), direction);
        true
    }
}
