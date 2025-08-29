pub use config::*;
pub use log::*;
pub use material_viewer::*;
pub use surface_editor::*;
pub use system_2d_viewer::*;

use super::State;

mod config;
mod log;
mod material_viewer;
mod surface_editor;
mod system_2d_viewer;

#[non_exhaustive]
pub enum TabKind {
    SurfaceEditor(SurfaceEditor),
    Log(Log),
    MaterialViewer(MaterialViewer),
    System2dViewer(System2dViewer),
    Config(Config),
}

pub struct Tab {
    id: usize,
    kind: TabKind,
}

pub struct TabViewer<'a> {
    state: &'a mut State,
}

impl<'a> TabViewer<'a> {
    pub fn new(state: &'a mut State) -> Self {
        Self { state }
    }
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab.kind {
            TabKind::SurfaceEditor(_) => "Surface Editor".into(),
            TabKind::Log(_) => "Log".into(),
            TabKind::MaterialViewer(_) => "Material Viewer".into(),
            TabKind::System2dViewer(_) => "System 2D Viewer".into(),
            TabKind::Config(_) => "Config".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| match &mut tab.kind {
                TabKind::SurfaceEditor(editor) => editor.ui(ui, self.state),
                TabKind::Log(log) => log.ui(ui, self.state),
                TabKind::MaterialViewer(viewer) => viewer.ui(ui, self.state),
                TabKind::System2dViewer(viewer) => viewer.ui(ui, self.state),
                TabKind::Config(config) => config.ui(ui, self.state),
            });
    }

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab.id)
    }
}

impl Tab {
    pub const fn new(id: usize, kind: TabKind) -> Self {
        Self { id, kind }
    }
}

impl TabKind {
    pub fn new_surface_editor() -> Self {
        TabKind::SurfaceEditor(SurfaceEditor::new())
    }

    pub fn new_log() -> Self {
        TabKind::Log(Log::new())
    }

    pub fn new_material_viewer() -> Self {
        TabKind::MaterialViewer(MaterialViewer::new())
    }

    pub fn new_system_2d_viewer() -> Self {
        TabKind::System2dViewer(System2dViewer::new())
    }

    pub fn new_config() -> Self {
        TabKind::Config(Config::new())
    }
}
