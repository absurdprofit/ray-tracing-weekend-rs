# Ray Tracing in One Weekend — Rust Implementation
My own Rust port of the *Ray Tracing in One Weekend* C++ project.  
The project renders simple 3D scenes using spheres, materials, lighting approximation, antialiasing, depth of field, and recursive ray scattering.

## Features
- PPM image output
- Camera with configurable aspect ratio and field of view
- Antialiasing through multiple samples per pixel
- Diffuse, metal, and dielectric/glass materials
- Recursive ray scattering
- Gamma correction
- Depth of field / defocus blur
- Random scene generation

## Requirements

- The Rustlang toolchain 'rustup'

## Build

```bash
cargo build --release
./target/release/ray-tracing-weekend-rs
```