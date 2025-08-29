use crate::glam::Vec3;

#[derive(Clone)]
pub struct Intersection {
    pub normal: Vec3,
    pub t: f32,
}
