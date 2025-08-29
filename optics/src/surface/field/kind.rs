use super::Field;
use crate::prelude::MaterialIndex;

macro_rules! impl_field_kind {
    ($type:ty) => {
        const _: () = assert!(core::mem::size_of::<$type>() == core::mem::size_of::<Field>());

        impl<'i> From<&'i mut Field> for &'i mut $type {
            fn from(value: &'i mut Field) -> Self {
                unsafe { &mut *(value.as_mut() as *mut u32 as *mut $type) }
            }
        }

        impl<'i> From<&'i Field> for &'i $type {
            fn from(value: &'i Field) -> Self {
                unsafe { &*(value.as_ref() as *const u32 as *const $type) }
            }
        }

        impl From<Field> for $type {
            fn from(value: Field) -> Self {
                unsafe { core::mem::transmute(value) }
            }
        }
    };
}

impl_field_kind!(u32);
impl_field_kind!(i32);
impl_field_kind!(f32);

// This is safe because the layout of `Field` is guaranteed to be the same as `Option<MaterialIndex>`.
impl_field_kind!(Option<MaterialIndex>);

const _: () =
    assert!(core::mem::size_of::<Field>() == core::mem::size_of::<Option<MaterialIndex>>());

impl<'i> From<&'i mut Field> for Option<&'i mut MaterialIndex> {
    fn from(value: &'i mut Field) -> Self {
        unsafe { &mut *(value.as_mut() as *mut u32 as *mut Option<MaterialIndex>) }.as_mut()
    }
}

impl<'i> From<&'i Field> for Option<&'i MaterialIndex> {
    fn from(value: &'i Field) -> Self {
        unsafe { &*(value.as_ref() as *const u32 as *const Option<MaterialIndex>) }.as_ref()
    }
}

impl From<Field> for usize {
    fn from(value: Field) -> Self {
        value.0 as Self
    }
}
