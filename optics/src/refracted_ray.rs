use crate::ray::Ray;

pub type RefractiveIndex = f32;

#[derive(Clone, encase::ShaderType)]
pub struct RefractedRay {
    pub ray: Ray,
    pub refractive_index: RefractiveIndex,
}

impl RefractedRay {
    pub const fn new(ray: Ray, refractive_index: RefractiveIndex) -> Self {
        Self {
            ray,
            refractive_index,
        }
    }
}

impl core::ops::Deref for RefractedRay {
    type Target = Ray;

    fn deref(&self) -> &Self::Target {
        &self.ray
    }
}
