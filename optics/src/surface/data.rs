use crate::surface::field::Field;

#[derive(Debug, Clone, Default)]
pub struct SurfaceData([Field; Self::LEN]);

impl core::ops::Deref for SurfaceData {
    type Target = [Field; Self::LEN];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for SurfaceData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SurfaceData {
    pub const LEN: usize = 32;

    pub fn with<T>(self, index: usize, value: T) -> Self
    where
        for<'a> &'a mut Field: Into<&'a mut T>,
    {
        let mut data = self;
        let field: &mut T = (&mut data[index]).into();
        *field = value;
        data
    }
}
