use egui::{DragValue, Response, Ui};
use optics::ray::Wavelength;

use crate::app::{formatting::Formatting, si};

pub fn wavelength(ui: &mut Ui, value: &mut Wavelength, fmt: &Formatting) -> Response {
    let prefix = fmt.length_prefix;

    let min = 1.0 * prefix.as_factor();
    let max = 1000.0 * prefix.as_factor();

    let mut new_value = (*value) * (prefix.as_factor() / si::Prefix::Micro.as_factor());

    let suffix = format!(" {}m", prefix.as_str());

    let response = ui.add(
        DragValue::new(&mut new_value)
            .suffix(suffix)
            .speed(0.05)
            .range(min..=max)
            .fixed_decimals(fmt.decimal_places)
            .custom_formatter(|value, _| {
                if value.is_infinite() {
                    "∞".into()
                } else {
                    format!("{value:.*}", fmt.decimal_places)
                }
            }),
    );

    *value = new_value / (prefix.as_factor() * si::Prefix::Micro.as_factor());

    response
}
