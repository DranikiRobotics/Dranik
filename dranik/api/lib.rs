//! The API for the dranik robot.

#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

pub(crate) mod threadsafe;
pub(crate) mod gamepad;
mod telemetry;
mod config;
mod error;
mod op;

/// Common traits and types used by the robot.
pub mod prelude {
    pub use super::config::RobotConfig;
    pub use super::gamepad::Gamepad;
    pub use super::telemetry::Telemetry;
    pub use super::threadsafe::ThreadSafeError;
    pub use super::op::Op;
}

pub use error::{HardwareError, Result, IO_OK};

/// The binary API for the robot.
#[allow(unused, missing_docs)]
#[cfg(feature = "bin")]
pub mod bin;
