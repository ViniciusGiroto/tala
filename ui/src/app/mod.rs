mod formatting;
mod log;
pub mod si;
mod state;
mod tabs;
mod widgets;

use std::collections::BTreeMap;

use egui::{self, FontData, FontDefinitions, FontId, TextStyle, ThemePreference};
use egui_dock::DockArea;
use log::{Log, LogLevel};
use state::State;
use tabs::{Tab, TabKind, TabViewer};

pub struct App {
    dock_state: egui_dock::DockState<Tab>,
    state: State,
    tab_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            dock_state: egui_dock::DockState::new(vec![
                Tab::new(0, TabKind::new_surface_editor()),
                Tab::new(1, TabKind::new_log()),
                Tab::new(2, TabKind::new_material_viewer()),
                Tab::new(3, TabKind::new_system_2d_viewer()),
            ]),
            state: Default::default(),
            tab_index: 3,
        }
    }

    pub fn open(&mut self, kind: TabKind) {
        self.tab_index += 1;
        self.dock_state
            .push_to_focused_leaf(Tab::new(self.tab_index, kind));
    }

    pub fn clear_logs(&mut self) {
        self.state.log.clear();
    }

    pub fn log(&mut self, level: LogLevel, title: impl Into<String>, message: impl Into<String>) {
        self.state.log.push(Log::new(level, title, message));
    }

    pub fn sync_and_run(&mut self) {
        let thickness = self.state.system.thickness();
        let power = self
            .state
            .system
            .power(self.state.system.wavelengths[0])
            .map_or_else(
                || "-".to_owned(),
                |power| format!("{}⁻¹", self.state.formatting.length(power)),
            );

        self.clear_logs();
        self.log(
            LogLevel::Info,
            "Measurements",
            format!(
                "Thickness: {}\nPower: {power}",
                self.state.formatting.length(thickness),
            ),
        );
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("foobar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Sync & Run").clicked() {
                    self.sync_and_run();
                }

                ui.menu_button("Open Window", |ui| {
                    if ui.button("Log").clicked() {
                        self.open(TabKind::new_log());
                    }

                    if ui.button("Material Viewer").clicked() {
                        self.open(TabKind::new_material_viewer());
                    }

                    if ui.button("Surface Editor").clicked() {
                        self.open(TabKind::new_surface_editor());
                    }

                    if ui.button("System 2D Viewer").clicked() {
                        self.open(TabKind::new_system_2d_viewer());
                    }

                    if ui.button("Config").clicked() {
                        self.open(TabKind::new_config());
                    }
                });
            });
        });

        DockArea::new(&mut self.dock_state)
            // .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut TabViewer::new(&mut self.state));
    }
}

/// Everything related to the UI must run in a separate thread.
pub fn run() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 480.0]),
        ..Default::default()
    };

    let result = eframe::run_native(
        "Tala",
        options,
        Box::new(|cc| {
            let mut fonts = FontDefinitions::empty();

            fonts.font_data.insert(
                "inter".to_owned(),
                std::sync::Arc::new(FontData::from_static(include_bytes!(
                    "../../assets/fonts/InterVariable.ttf"
                ))),
            );

            fonts.font_data.insert(
                "inter-mono".to_owned(),
                std::sync::Arc::new(FontData::from_static(include_bytes!(
                    "../../assets/fonts/InterVariable.ttf"
                ))),
            );

            fonts.font_data.insert(
                "inter-italic".to_owned(),
                std::sync::Arc::new(FontData::from_static(include_bytes!(
                    "../../assets/fonts/InterVariable-Italic.ttf"
                ))),
            );

            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .push("inter".to_owned());
            fonts
                .families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .push("inter-mono".to_owned());

            let text_styles: BTreeMap<_, _> = [
                (
                    TextStyle::Body,
                    FontId::new(16.0, egui::FontFamily::Proportional),
                ),
                (
                    TextStyle::Heading,
                    FontId::new(16.0, egui::FontFamily::Proportional),
                ),
                (
                    TextStyle::Small,
                    FontId::new(14.0, egui::FontFamily::Proportional),
                ),
                (
                    TextStyle::Button,
                    FontId::new(16.0, egui::FontFamily::Proportional),
                ),
                (
                    TextStyle::Monospace,
                    FontId::new(16.0, egui::FontFamily::Monospace),
                ),
            ]
            .into();

            let mut style = egui::style::Style::default();

            style.text_styles = text_styles;

            cc.egui_ctx.set_fonts(fonts);
            cc.egui_ctx.set_style(style);

            cc.egui_ctx.set_theme(ThemePreference::System);

            Ok(Box::new(App::new()))
        }),
    );

    match result {
        Ok(_) => Ok(()),
        Err(_err) => todo!("implement error handling"),
    }
}
