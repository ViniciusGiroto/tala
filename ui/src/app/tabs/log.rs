use crate::app::State;

pub struct Log;

impl Log {
    pub fn new() -> Self {
        Log
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        for (i, log) in state.log.iter().enumerate() {
            ui.push_id(i, |ui| {
                ui.collapsing(&log.title, |ui| {
                    ui.label(&log.message);
                });
            });
        }
    }
}
