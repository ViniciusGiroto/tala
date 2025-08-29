use egui::Sense;
use egui_extras::{Column, TableBuilder};
use optics::surface::SurfaceData;

use crate::app::{State, widgets::SurfaceRow};

pub struct SurfaceEditor {
    pub row: usize,
}

impl SurfaceEditor {
    pub fn new() -> Self {
        Self { row: 0 }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .column(Column::auto())
            .column(Column::initial(128.0))
            .columns(Column::initial(128.0), SurfaceData::LEN)
            .sense(Sense::all());

        table
            .header(20.0, |mut header| {
                let surface = &state.system.surfaces[self.row];

                for name in ["#", "Type"] {
                    header.col(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.strong(name);
                        });
                    });
                }

                for (i, field) in surface.kind().fields().iter().enumerate() {
                    header.col(|ui| {
                        ui.centered_and_justified(|ui| {
                            match field {
                                Some(name) => ui.strong(match *name {
                                    "thickness" => "Thickness",
                                    "material_index" => "Material",
                                    "curvature" => "Curvature",
                                    "semi_diameter" => "Semi-diameter",
                                    "translation_x" => "Translation X",
                                    "translation_y" => "Translation Y",
                                    "translation_z" => "Translation Z",
                                    "rotation_order" => "Rotation Order",
                                    "rotation_x" => "Rotation X",
                                    "rotation_y" => "Rotation Y",
                                    "rotation_z" => "Rotation Z",
                                    _ => name,
                                }),
                                None => ui.strong(format!("Arg[{i}]")),
                            };
                        });
                    });
                }
            })
            .body(|body| {
                body.rows(20.0, state.system.surfaces.len(), |mut row| {
                    SurfaceRow::new().show(&mut row, self, state);
                });
            });
    }
}
