use core::fmt::{Debug, Display};

/// A type that allows sending telemetry data to the driver control station
/// 
/// Although this is a trait, it is not meant to be implemented by the user.
/// Instead, it is implemented by this crate.
pub trait Telemetry: Clone + PartialEq + Send + Sync {
    /// Sends a debug message to the driver control station
    /// 
    /// It will not be displayed to the driver control station if the robot is not in debug mode.
    /// 
    /// If this fails, it will fail silently.
    fn debug<T: Debug>(&self, message: T);
    /// Sends a message to the driver control station
    /// 
    /// This will always be displayed to the driver control station.
    /// 
    /// If this fails, it will fail silently.
    fn send<T: Display>(&self, message: T);
}
