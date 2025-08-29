use crate::surface::SurfaceData;

macro_rules! assert_field {
    ($index:expr) => {
        const {
            let value = $index;
            assert!((value as usize) < SurfaceData::LEN, "Invalid field index");
            $index
        }
    };
}

pub const THICKNESS: usize = assert_field!(0);
pub const MATERIAL_INDEX: usize = assert_field!(1);
pub const CURVATURE: usize = assert_field!(2);
pub const SEMI_DIAMETER: usize = assert_field!(3);

pub const ROTATION_ORDER: usize = assert_field!(4);

pub const TRANSLATION_X: usize = assert_field!(5);
pub const TRANSLATION_Y: usize = assert_field!(6);
pub const TRANSLATION_Z: usize = assert_field!(7);

pub const ROTATION_X: usize = assert_field!(8);
pub const ROTATION_Y: usize = assert_field!(9);
pub const ROTATION_Z: usize = assert_field!(10);
