use crate::{
    colour::Colour,
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    vec3::{random_unit_vector, reflect, unit_vector},
};

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.direction().clone(), rec.normal.clone());
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(rec.point.clone(), reflected);
        *attenuation = self.albedo.clone();
        true
    }
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}
