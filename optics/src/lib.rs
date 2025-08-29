pub mod intersection;
pub mod material;
pub mod ray;
pub mod refracted_ray;
pub mod surface;
pub mod system;

// CPU specific implementations
#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

// GPU specific implementations
#[cfg(target_arch = "spirv")]
mod gpu;
#[cfg(target_arch = "spirv")]
pub use gpu::*;

pub mod prelude {
    pub use crate::{
        intersection::Intersection,
        material::MaterialIndex,
        ray::{Ray, Wavelength},
        surface::Surface,
        system::System,
    };
}
