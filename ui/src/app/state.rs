use optics::system::System;

use crate::app::{formatting::Formatting, log::Log};

#[derive(Debug)]
pub struct State {
    pub(crate) system: System,
    pub(crate) log: Vec<Log>,
    pub(crate) formatting: Formatting,
}

impl Default for State {
    fn default() -> Self {
        Self {
            system: System::default(),
            log: Vec::new(),
            formatting: Formatting::default(),
        }
    }
}
