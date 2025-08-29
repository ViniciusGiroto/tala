use crate::{
    glam::{Mat4, vec3},
    surface::*,
};

pub fn transformation_matrix(data: &SurfaceData) -> Mat4 {
    let translation = vec3(
        data[TRANSLATION_X].into(),
        data[TRANSLATION_Y].into(),
        data[TRANSLATION_Z].into(),
    );
    let rotation = vec3(
        data[ROTATION_X].into(),
        data[ROTATION_Y].into(),
        data[ROTATION_Z].into(),
    );
    let rotation_order = data[ROTATION_ORDER].into();

    let rotation_x = Mat4::from_rotation_x(rotation.x.to_radians());
    let rotation_y = Mat4::from_rotation_y(rotation.y.to_radians());
    let rotation_z = Mat4::from_rotation_z(rotation.z.to_radians());

    //  0 => XYZ
    //  1 => XZY
    //  2 => YXZ
    //  3 => YZX
    //  4 => ZXY
    //  5 => ZYX

    let ra = match rotation_order {
        0 | 1 => rotation_x,
        2 | 3 => rotation_y,
        4 | 5 => rotation_z,
        _ => panic!("Invalid rotation order"),
    };

    let rb = match rotation_order {
        2 | 4 => rotation_x,
        0 | 5 => rotation_y,
        1 | 3 => rotation_z,
        _ => panic!("Invalid rotation order"),
    };

    let rc = match rotation_order {
        3 | 5 => rotation_x,
        1 | 4 => rotation_y,
        0 | 2 => rotation_z,
        _ => panic!("Invalid rotation order"),
    };

    let translation = Mat4::from_translation(translation);

    return rc * rb * ra * translation;
}
