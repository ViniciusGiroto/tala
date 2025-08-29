use crate::surface::*;

pub mod coordinate_break;
pub mod image;
pub mod object;
pub mod spherical;

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[repr(u32)]
pub enum SurfaceKind {
    Object,
    Image,
    Spherical,
    CoordinateBreak,
}

#[cfg(not(target_arch = "spirv"))]
impl SurfaceKind {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Object => "Object",
            Self::Image => "Image",
            Self::Spherical => "Spherical",
            Self::CoordinateBreak => "CoordinateBreak",
        }
    }

    pub const fn fields(&self) -> &'static [Option<&'static str>; SurfaceData::LEN] {
        match self {
            Self::Object => {
                &const {
                    let mut fields = [None; SurfaceData::LEN];
                    fields[THICKNESS] = Some("thickness");
                    fields[MATERIAL_INDEX] = Some("material_index");
                    fields[SEMI_DIAMETER] = Some("semi_diameter");
                    fields
                }
            }
            Self::Image => {
                &const {
                    let mut fields = [None; SurfaceData::LEN];
                    fields[SEMI_DIAMETER] = Some("semi_diameter");
                    fields
                }
            }
            Self::Spherical => {
                &const {
                    let mut fields = [None; SurfaceData::LEN];
                    fields[CURVATURE] = Some("curvature");
                    fields[THICKNESS] = Some("thickness");
                    fields[MATERIAL_INDEX] = Some("material_index");
                    fields[SEMI_DIAMETER] = Some("semi_diameter");
                    fields
                }
            }
            Self::CoordinateBreak => {
                &const {
                    let mut fields = [None; SurfaceData::LEN];
                    fields[TRANSLATION_X] = Some("translation_x");
                    fields[TRANSLATION_Y] = Some("translation_y");
                    fields[TRANSLATION_Z] = Some("translation_z");
                    fields[ROTATION_ORDER] = Some("rotation_order");
                    fields[ROTATION_X] = Some("rotation_x");
                    fields[ROTATION_Y] = Some("rotation_y");
                    fields[ROTATION_Z] = Some("rotation_z");
                    fields
                }
            }
        }
    }
}
