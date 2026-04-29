use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn random_double() -> f64 {
  const RAND_MAX: u16 = 32767;
  rand::thread_rng().gen_range(0..=RAND_MAX) as f64 / (RAND_MAX as f64 + 1.0)
}

pub fn bounded_random_double(min: f64, max: f64) -> f64 {
  min + (max - min) * random_double()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
  degrees * PI / 180.0
}