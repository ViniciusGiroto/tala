use std::f64;

use egui::{Color32, Stroke};
use egui_plot::{Line, LineStyle, Plot, PlotPoints};
use optics::{
    glam::{Vec3, Vec3Swizzles, vec3},
    surface::{CURVATURE, SEMI_DIAMETER, SurfaceKind, THICKNESS},
};

use crate::app::State;

pub struct System2dViewer {}

impl System2dViewer {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let plot = Plot::new("system_2d_viewer")
            .label_formatter(|name, value| {
                if name.is_empty() {
                    format!(
                        "y: {:.*}\nz: {:.*}",
                        state.formatting.decimal_places,
                        value.y,
                        state.formatting.decimal_places,
                        value.x,
                    )
                } else {
                    format!(
                        "{name}\ny: {:.*}\nz: {:.*}",
                        state.formatting.decimal_places,
                        value.y,
                        state.formatting.decimal_places,
                        value.x,
                    )
                }
            })
            .data_aspect(1.0)
            .show_grid([false, false]);

        plot.show(ui, |plot| {
            let mut axis_points = Vec::new();
            let mut lines = Vec::new();

            axis_points.push([0.0, 0.0]);
            // plot.vline(VLine::new("y", 0.0).stroke(Stroke::new(1.0, Color32::GREEN)));

            for (i, (surface, transform)) in state.system.surfaces().enumerate() {
                let data = surface.data();

                {
                    let [x, y] = transform.project_point3(Vec3::ZERO).zy().to_array();
                    axis_points.push([x as f64, y as f64]);
                }

                match surface.kind() {
                    SurfaceKind::CoordinateBreak => {}
                    SurfaceKind::Image => {}
                    SurfaceKind::Object => {
                        let distance: f32 = data[THICKNESS].into();
                        let semi_diameter: f32 = data[SEMI_DIAMETER].into();

                        if distance.is_finite() {
                            axis_points[0][0] = -distance as f64;

                            lines.push(
                                Line::new(
                                    format!("Object"),
                                    vec![
                                        [-distance as f64, -semi_diameter as f64],
                                        [-distance as f64, semi_diameter as f64],
                                    ],
                                )
                                .stroke(Stroke::new(1.0, Color32::WHITE)),
                            );
                        } else {
                            axis_points[0][0] = 10000.0;
                        }
                    }
                    SurfaceKind::Spherical => {
                        const N: usize = 512;

                        let radius = {
                            let curvature: f32 = data[CURVATURE].into();
                            curvature.recip()
                        };

                        let semi_diameter = {
                            let semi_diameter: f32 = data[SEMI_DIAMETER].into();

                            if !semi_diameter.is_finite() || semi_diameter <= 0.0 {
                                radius.abs() as f64
                            } else {
                                semi_diameter.min(radius.abs()) as f64
                            }
                        };

                        /*
                        let points = PlotPoints::from_parametric_callback(
                            |angle| {
                                let [x, y] = transform
                                    .project_point3(
                                        (radius
                                            * (glam::Vec2::from_angle(angle as f32)
                                                + glam::Vec2::X))
                                            .extend(0.0)
                                            .zyx(),
                                    )
                                    .zy()
                                    .as_dvec2()
                                    .to_array();
                                (x, y)
                            },
                            0.0..=f64::consts::TAU,
                            N,
                        );

                        lines.push(
                            Line::new(format!("Surface {i}"), points)
                                .stroke(Stroke::new(1.0, Color32::from_white_alpha(31)))
                                .style(LineStyle::dashed_loose())
                                .allow_hover(false),
                        );
                        */

                        let points = PlotPoints::from_parametric_callback(
                            |y| {
                                let x = -radius as f64 * (-1.0 + (y / radius as f64).asin().cos());
                                let [x, y] = transform
                                    .project_point3(vec3(0.0, y as f32, x as f32))
                                    .zy()
                                    .as_dvec2()
                                    .to_array();
                                (x, y)
                            },
                            -semi_diameter..=semi_diameter,
                            N,
                        );

                        lines.push(
                            Line::new(format!("Surface {i}"), points)
                                .stroke(Stroke::new(1.0, Color32::WHITE)),
                        );
                    }
                    _ => {}
                }
            }

            plot.line(
                Line::new(format!("Optical Axis"), axis_points)
                    .stroke(Stroke::new(1.0, Color32::DARK_BLUE))
                    .style(LineStyle::dashed_loose()),
            );

            for line in lines {
                plot.line(line);
            }
        });
    }
}
