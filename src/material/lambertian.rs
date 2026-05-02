// class lambertian : public material {
//   public:
//     lambertian(const colour& albedo) : albedo(albedo) {}

//     bool scatter(
//       const ray& r_in,
//       const hit_record& rec,
//       colour& attenuation,
//       ray& scattered
//     ) const override {
//       auto scatter_direction = rec.normal + random_unit_vector();

//       if (scatter_direction.near_zero())
//         scatter_direction = rec.normal;

//       scattered = ray(rec.p, scatter_direction);
//       attenuation = albedo;
//       return true;
//     }

//   private:
//     colour albedo;
// };

use crate::{colour::Colour, hittable::HitRecord, ray::Ray, vec3::random_unit_vector};

pub struct Lambertian {
  albedo: Colour,
}

impl Lambertian {
  pub fn new(albedo: Colour) -> Self {
    Self { albedo }
  }

  pub fn scatter(
      &self,
      r_in: Ray,
      rec: &HitRecord,
      attenuation: &mut Colour,
      scattered: &mut Ray
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