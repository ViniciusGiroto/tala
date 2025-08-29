#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug)]
pub struct Log {
    pub level: LogLevel,
    pub title: String,
    pub message: String,
}

impl Log {
    pub fn new(level: LogLevel, title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level,
            title: title.into(),
            message: message.into(),
        }
    }
}
