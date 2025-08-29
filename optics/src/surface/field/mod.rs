mod index;
mod kind;

pub use index::*;

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Field(pub(crate) u32);

impl Field {
    pub(crate) const fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }

    pub(crate) const fn as_ref(&self) -> &u32 {
        &self.0
    }
}
