/// A type that allows sending telemetry data to the driver control station
/// 
/// Although this is a trait, it is not meant to be implemented by the user.
/// Instead, it is implemented by this crate.
pub trait Telemetry {
    /// Sends a message to the driver control station
    /// 
    /// If this fails, it will fail silently.
    fn send(&self, message: TelemetryMessage);

    /// Sends a log message to the driver control station
    #[inline]
    fn log(&self, message: String) {
        self.send(TelemetryMessage::Log(message));
    }
    /// Sends a debug message to the driver control station
    #[inline]
    fn debug(&self, message: String) {
        self.send(TelemetryMessage::Debug(message));
    }
    /// Sends an error message to the driver control station
    #[inline]
    fn error(&self, message: String) {
        self.send(TelemetryMessage::Error(message));
    }
}

/// A message that can be sent to the driver control station
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TelemetryMessage {
    /// A standard log message
    Log(String),
    /// A debug message
    Debug(String),
    /// An error message
    Error(String),
}

impl ToString for TelemetryMessage {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            TelemetryMessage::Log(message) => format!("[LOG] {}", message),
            TelemetryMessage::Debug(message) => format!("[DEBUG] {}", message),
            TelemetryMessage::Error(message) => format!("[ERROR] {}", message),
        }
    }
}

impl From<TelemetryMessage> for String {
    #[inline]
    fn from(message: TelemetryMessage) -> Self {
        match message {
            TelemetryMessage::Log(msg) => msg,
            TelemetryMessage::Debug(msg) => msg,
            TelemetryMessage::Error(msg) => msg,
        }
    }
}
