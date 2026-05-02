use crate::{interval::Interval, vec3::Vec3};
use std::io::{BufWriter, Write, Result};

pub type Colour = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
  if linear_component > 0.0 {
    return linear_component.sqrt();
  }

  return 0.0;
}

const INTENSITY: Interval = Interval {
  min: 0.000,
  max: 0.999
};

pub fn write_colour<W: Write>(writer: &mut BufWriter<W>, pixel_colour: Colour) -> Result<()> {
  let r = pixel_colour.x();
  let g = pixel_colour.y();
  let b = pixel_colour.z();

  let r = linear_to_gamma(r);
  let g = linear_to_gamma(g);
  let b = linear_to_gamma(b);
  
  // Translate the [0,0.999] component values to the byte range [0,255]
  let rbyte = (256.0 * INTENSITY.clamp(r)) as u8;
  let gbyte = (256.0 * INTENSITY.clamp(g)) as u8;
  let bbyte = (256.0 * INTENSITY.clamp(b)) as u8;

  // Write out the pixel colour components.
  let space = b" ";
  writer.write_all(rbyte.to_string().as_bytes())?;
  writer.write_all(space)?;
  writer.write_all(gbyte.to_string().as_bytes())?;
  writer.write_all(space)?;
  writer.write_all(bbyte.to_string().as_bytes())?;
  writer.write_all(b"\n")?;

  Ok(())
}