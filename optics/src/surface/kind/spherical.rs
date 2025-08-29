#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{
    glam::{Mat4, Vec3},
    intersection::Intersection,
    refracted_ray::RefractedRay,
    surface::{CURVATURE, SurfaceData, THICKNESS},
};

pub fn transformation_matrix(data: &SurfaceData) -> Mat4 {
    let thickness: f32 = data[THICKNESS].into();
    Mat4::from_translation(thickness * Vec3::Z)
}

pub fn intersect(
    data: &SurfaceData,
    refracted_ray: &RefractedRay,
    transform: &Mat4,
) -> Intersection {
    let curvature: f32 = data[CURVATURE].into();

    if curvature == 0.0 {
        todo!();
        /*
        return Intersection {
            ray: ray.clone(),
            normal: Vec3::NEG_Z,
            t: (z - ray.origin.z) / ray.direction.z,
        };
        */
    }

    let radius = curvature.recip();
    // perhaps this could be changed to transform_point3
    let center = transform.project_point3(radius * Vec3::Z);

    let a = 1.0; // 1.0 because ray.direction is normalized
    let b = 2.0 * refracted_ray.direction.dot(refracted_ray.origin - center);
    let c = (refracted_ray.origin - center).dot(refracted_ray.origin - center) - radius * radius;

    let delta = (b * b - 4.0 * a * c).sqrt();

    let t1 = (-b - delta) / (2.0 * a);
    let t2 = (-b + delta) / (2.0 * a);

    let s = radius.signum();
    let t = if radius < 0.0 { t2 } else { t1 };
    let normal = (refracted_ray.direction * t + refracted_ray.origin - center).normalize() * s;

    return Intersection { normal, t };
}
