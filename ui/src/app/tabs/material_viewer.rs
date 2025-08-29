use egui::ComboBox;
use egui_plot::{Line, Plot, PlotPoints};

use crate::app::State;

pub struct MaterialViewer {
    material: usize,
}

impl MaterialViewer {
    pub fn new() -> Self {
        Self { material: 0 }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        ui.horizontal(|ui| {
            // ui.allocate_space(vec2(0.0, 20.0));
            ui.label("Material:");
            ComboBox::from_id_salt("material").show_index(
                ui,
                &mut self.material,
                state.system.materials.len(),
                |i| state.system.materials[i].name(),
            );
        });

        ui.separator();

        let available_size = ui.available_size();
        let available_aspect_ratio = available_size.x / available_size.y;

        let mut plot = Plot::new("refractive_index")
            .x_axis_label("Wavelength (μm)")
            .y_axis_label("n")
            .label_formatter(|name, value| {
                if name.is_empty() {
                    "".to_owned()
                } else {
                    format!(
                        "{:.*} μm\nn = {:.*}",
                        state.formatting.decimal_places,
                        value.x,
                        state.formatting.decimal_places,
                        value.y,
                    )
                }
            })
            .view_aspect(1.0);

        plot = if available_aspect_ratio > 1.0 {
            plot.height(available_size.y.max(320.0))
        } else {
            plot.width(available_size.x.max(320.0))
        };

        let material = &state.system.materials[self.material];
        let n: PlotPoints = (0..1000)
            .map(|i| {
                let wavelength = (i as f32) * 0.01;
                [
                    wavelength as f64,
                    material.refractive_index(wavelength) as f64,
                ]
            })
            .collect();

        plot.show(ui, |ui| {
            ui.line(Line::new("n", n));
        });
    }
}
