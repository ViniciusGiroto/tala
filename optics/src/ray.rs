use crate::glam::Vec3;

/// Wavelength of light in nanometers.
pub type Wavelength = f32;

#[derive(Clone, Copy, encase::ShaderType)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub wavelength: Wavelength,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, wavelength: Wavelength) -> Self {
        Self {
            origin,
            direction,
            wavelength,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
