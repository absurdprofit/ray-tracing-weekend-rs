use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::common::{bounded_random_double, random_double};

#[derive(Clone, Debug)]
pub struct Vec3 {
  pub e: (f64, f64, f64)
}

impl Vec3 {
  pub fn new() -> Vec3 {
    Vec3 {
      e: (0.0, 0.0, 0.0)
    }
  }

  pub fn from_tuple(e: (f64, f64, f64)) -> Vec3 {
    Vec3 {
      e
    }
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2
  }

  pub fn near_zero(&self) -> bool {
    let s = 1e-8;
    (self.e.0.abs() < s) && (self.e.1.abs() < s) && (self.e.2.abs() < s)
  }

  pub fn random() -> Vec3 {
    Vec3::from_tuple((random_double(), random_double(), random_double()))
  }

  pub fn bounded_random(min: f64, max: f64) -> Vec3 {
    Vec3::from_tuple((
      bounded_random_double(min, max),
      bounded_random_double(min, max),
      bounded_random_double(min, max)
    ))
  }

  pub fn x(self: &Self) -> f64 {
    self.e.0
  }

  pub fn y(self: &Self) -> f64 {
    self.e.1
  }

  pub fn z(self: &Self) -> f64 {
    self.e.2
  }
}

pub type Point3 = Vec3;

impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Vec3::from_tuple((-self.e.0, -self.e.1, -self.e.2))
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self::from_tuple((
      self.x() - other.x(),
      self.y() - other.y(),
      self.z() - other.z(),
    ))
  }
}

impl Mul<Vec3> for Vec3 {
  // The multiplication of rational numbers is a closed operation.
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    Self::from_tuple((
      self.x() * other.x(),
      self.y() * other.y(),
      self.z() * other.z()
    ))
  }
}

impl Mul<f64> for Vec3 {
  // The multiplication of rational numbers is a closed operation.
  type Output = Self;

  fn mul(self, other: f64) -> Self {
    Self::from_tuple((
      other * self.x(),
      other * self.y(),
      other * self.z()
    ))
  }
}

impl Mul<Vec3> for f64 {
  // The multiplication of rational numbers is a closed operation.
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Self::Output {
    other * self
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, other: Self) {
    self.e.0 += other.e.0;
    self.e.1 += other.e.1;
    self.e.2 += other.e.2;
  }
}

impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, other: f64) {
    self.e.0 *= other;
    self.e.1 *= other;
    self.e.2 *= other;
  }
}

impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, other: f64) {
    *self *= 1.0 / other
  }
}

impl Div<f64> for Vec3 {
  type Output = Self;

  fn div(self, other: f64) -> Self::Output {
    if other == 0.0 {
      panic!("Cannot divide by zero-valued `Vec3`!");
    }

    (1.0 / other) * self
  }
}

impl Add for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Vec3::from_tuple((
      self.e.0 + other.e.0,
      self.e.1 + other.e.1,
      self.e.2 + other.e.2
    ))
  }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
  u.x() * v.x()
  + u.y() * v.y()
  + u.z() * v.z()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
  Vec3::from_tuple((
    u.y() * v.z() - u.z() * v.y(),
    u.z() * v.x() - u.x() * v.z(),
    u.x() * v.y() - u.y() * v.x()
  ))
}

pub fn unit_vector(v: Vec3) -> Vec3 {
  let length = v.length();
  v / length
}

pub fn random_in_unit_disk() -> Vec3 {
  loop {
    let p = Vec3::from_tuple((
      bounded_random_double(-1.0, 1.0),
      bounded_random_double(-1.0, 1.0),
      0.0
    ));
    if p.length_squared() < 1.0 {
      return p;
    }
  };
}

pub fn random_unit_vector() -> Vec3 {
  loop {
    let p = Vec3::bounded_random(-1.0, 1.0);
    let lensq = p.length_squared();
    if 1e-160 < lensq && lensq <= 1.0 {
      return p / lensq.sqrt();
    }
  };
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
  let on_unit_sphere = random_unit_vector();
  if dot(&on_unit_sphere, normal) > 0.0 {
    return on_unit_sphere;
  } else {
    return -on_unit_sphere;
  }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  let dot_product = dot(&v, &n);
  v - 2.0 * dot_product * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
  let cos_theta = dot(&-uv.clone(), &n).min(1.0);
  let r_out_perp = etai_over_etat * (uv + cos_theta * n.clone());
  let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
  r_out_perp + r_out_parallel
}