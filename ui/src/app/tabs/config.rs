use crate::app::{State, widgets};

pub struct Config;

impl Config {
    pub fn new() -> Self {
        Config
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        ui.label("Medium:");
        widgets::material_index(ui, &mut state.system.medium, &state.system.materials);

        ui.separator();

        ui.label("Reference wavelength:");
        widgets::wavelength(ui, &mut state.system.wavelengths[0], &state.formatting);

        ui.separator();

        ui.label("Decimal places:");
        ui.add(egui::Slider::new(
            &mut state.formatting.decimal_places,
            1..=16,
        ));

        // ui.
    }
}
