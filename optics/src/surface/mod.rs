mod data;
mod field;
mod kind;

// pub use coordinate_break::CoordinateBreak;
pub use data::SurfaceData;
pub use field::*;
use glam::Mat4;
pub use kind::*;

use crate::{prelude::Intersection, refracted_ray::RefractedRay};

/// Surfaces are arrays of u32. Each SurfaceKind must be responsible for its own data logic.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Surface {
    pub(crate) kind: SurfaceKind,
    pub(crate) data: SurfaceData,
}

impl Default for Surface {
    fn default() -> Self {
        Self::new(SurfaceKind::Spherical, SurfaceData::default())
    }
}

impl Surface {
    pub const fn new(kind: SurfaceKind, data: SurfaceData) -> Self {
        Self { kind, data }
    }

    pub const fn kind(&self) -> SurfaceKind {
        self.kind
    }

    pub const fn data(&self) -> &SurfaceData {
        &self.data
    }

    pub const fn data_mut(&mut self) -> &mut SurfaceData {
        &mut self.data
    }

    pub fn intersect(&self, ray: &RefractedRay, transform: &Mat4) -> Intersection {
        match self.kind {
            SurfaceKind::Spherical => spherical::intersect(&self.data, ray, transform),
            SurfaceKind::CoordinateBreak => todo!(),
            SurfaceKind::Image => todo!(),
            SurfaceKind::Object => todo!(),
        }
    }
}
