use egui::{ComboBox, DragValue, Layout, Response, Ui};
use egui_extras::TableRow;
use optics::surface::{Field, SurfaceKind};

use crate::app::{
    State, formatting::Formatting, si, tabs::SurfaceEditor, widgets::material_index_optional,
};

pub struct SurfaceRow;

impl SurfaceRow {
    pub const fn new() -> Self {
        Self
    }

    pub fn show(
        &'_ mut self,
        row: &'_ mut TableRow<'_, '_>,
        editor: &mut SurfaceEditor,
        state: &mut State,
    ) {
        let row_index = row.index();

        if row_index >= state.system.surfaces.len() {
            return;
        }

        row.set_selected(row_index == editor.row);
        row.set_overline(row_index == 1 || row_index == state.system.surfaces.len() - 1);

        let surface = &mut state.system.surfaces[row_index];
        let kind = surface.kind();
        let data = surface.data_mut();

        // Index
        row.col(|ui| {
            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                ui.label(row_index.to_string());
            });
        });

        // Suface type
        row.col(|ui| {
            ui.label(kind.name());
        });

        for (index, field) in kind.fields().iter().enumerate() {
            let mut field_data = &mut data[index];

            row.col(|ui| {
                ui.with_layout(Layout::default().with_cross_justify(true), |ui| {
                    let response = match field {
                        Some("curvature") => curvature(ui, &mut field_data, &state.formatting),
                        Some(
                            "thickness" | "semi_diameter" | "translation_x" | "translation_y"
                            | "translation_z",
                        ) => length(ui, &mut field_data, &state.formatting),
                        Some("rotation_x" | "rotation_y" | "rotation_z") => {
                            angle(ui, &mut field_data, &state.formatting)
                        }
                        Some("rotation_order") => rotation_order(ui, &mut field_data),
                        Some("material_index") => material_index_optional(
                            ui,
                            field_data.into(),
                            &state.system.materials,
                            if kind == SurfaceKind::Object {
                                "Medium"
                            } else {
                                "Previous"
                            },
                        ),
                        Some(_) => ui.label("INVALID"),
                        None => ui.label("-"),
                    };

                    if response.clicked() | response.gained_focus() {
                        editor.row = row_index;
                    }
                });
            });
        }

        if row.response().clicked() | row.response().gained_focus() {
            editor.row = row_index;
        }

        row.response().context_menu(|ui| {
            if kind != SurfaceKind::Object {
                if ui.button("Add surface before").clicked() {
                    state.system.surfaces.insert(row_index, Default::default());
                }
            }

            if kind != SurfaceKind::Image {
                if ui.button("Add surface after").clicked() {
                    state
                        .system
                        .surfaces
                        .insert(row_index + 1, Default::default());
                }
            }

            if kind != SurfaceKind::Object && kind != SurfaceKind::Image {
                ui.separator();

                if ui.button("Duplicate surface").clicked() {
                    let new_surface = state.system.surfaces[row_index].clone();
                    state.system.surfaces.insert(row_index + 1, new_surface);
                }

                if ui.button("Delete surface").clicked() {
                    state.system.surfaces.remove(row_index);
                    editor.row = editor.row.min(state.system.surfaces.len() - 1);
                }
            }
        });
    }
}

fn curvature(ui: &mut Ui, field: &mut Field, fmt: &Formatting) -> Response {
    let curvature: &mut f32 = field.into();

    let factor = fmt.length_prefix.as_factor() / si::Prefix::Milli.as_factor();
    let suffix = format!(" {}m", fmt.length_prefix.as_str());

    let mut value = curvature.recip() * factor;
    let response = ui.add(
        DragValue::new(&mut value)
            .suffix(suffix)
            .speed(0.05)
            .fixed_decimals(3)
            .custom_parser(|input| input.parse::<f64>().ok().or(Some(f64::INFINITY)))
            .custom_formatter(|value, _| {
                if value.is_infinite() {
                    "∞".into()
                } else {
                    format!("{value:+4.*}", fmt.decimal_places)
                }
            }),
    );

    *curvature = value.recip() / factor;
    response
}

fn length(ui: &mut Ui, field: &mut Field, fmt: &Formatting) -> Response {
    let length: &mut f32 = field.into();

    let factor = fmt.length_prefix.as_factor() / si::Prefix::Milli.as_factor();
    let suffix = format!(" {}m", fmt.length_prefix.as_str());

    let mut value = *length * factor;
    let response = ui.add(
        DragValue::new(&mut value)
            .suffix(suffix)
            .speed(0.05)
            .fixed_decimals(fmt.decimal_places)
            .custom_parser(|input| input.parse::<f64>().ok().or(Some(f64::INFINITY)))
            .custom_formatter(|value, _| {
                if value.is_infinite() {
                    "∞".into()
                } else {
                    format!("{value:+4.*}", fmt.decimal_places)
                }
            }),
    );

    *length = value / factor;
    response
}

fn angle(ui: &mut Ui, field: &mut Field, fmt: &Formatting) -> Response {
    let angle: &mut f32 = field.into();

    ui.add(
        DragValue::new(angle)
            .suffix("°")
            .speed(0.05)
            .range(-360.0..=360.0)
            .fixed_decimals(fmt.decimal_places)
            .custom_formatter(|value, _| {
                if value.is_infinite() {
                    "∞".into()
                } else {
                    format!("{value:+4.*}", fmt.decimal_places)
                }
            }),
    )
}

fn rotation_order(ui: &mut Ui, field: &mut Field) -> Response {
    const ROTATION_ORDER_OPTIONS: &[&str] = &["XYZ", "XZY", "YXZ", "YZX", "ZXY", "ZYX"];
    let order: &mut u32 = field.into();
    let mut picked: usize = *order as usize;

    let response = ComboBox::from_id_salt("rotation_order")
        .width(ui.available_width())
        .show_index(ui, &mut picked, ROTATION_ORDER_OPTIONS.len(), |i| {
            ROTATION_ORDER_OPTIONS[i]
        });

    *order = picked as u32;
    response
}
