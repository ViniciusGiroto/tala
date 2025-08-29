use crate::app::si;

#[derive(Debug)]
pub struct Formatting {
    pub decimal_places: usize,
    pub length_prefix: si::Prefix,
}

impl Default for Formatting {
    fn default() -> Self {
        Formatting {
            decimal_places: 3,
            length_prefix: si::Prefix::Milli,
        }
    }
}

impl Formatting {
    pub fn length(&self, value: f32) -> String {
        let value = value * (self.length_prefix.as_factor() / si::Prefix::Milli.as_factor());

        format!(
            "{value:.*} {}m",
            self.decimal_places,
            self.length_prefix.as_str()
        )
    }
}
